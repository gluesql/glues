mod args;
mod proxy_access;
pub mod state;

pub use args::{ServerArgs, StorageCommand, parse_args};

use {
    axum::{
        Json, Router,
        body::Body,
        extract::State,
        http::{Method, Request, StatusCode, header::AUTHORIZATION},
        middleware::{Next, from_fn},
        response::Response,
        routing::{get, post},
    },
    color_eyre::Result,
    glues_core::backend::{
        CoreBackend,
        local::Db,
        proxy::{ProxyServer, request::ProxyRequest, response::ProxyResponse},
    },
    std::sync::Arc,
    tokio::{net::TcpListener, signal},
    tower_http::cors::{Any, CorsLayer},
    tracing::{error, info, warn},
    tracing_subscriber::EnvFilter,
};

use state::ServerState;

pub async fn run(args: ServerArgs) -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_target(false)
        .init();

    let ServerArgs {
        listen,
        auth_token,
        allowed_directory,
        storage,
    } = args;

    let backend = build_backend(storage).await?;
    let server = ProxyServer::new(backend);
    let state = Arc::new(ServerState::new(server, allowed_directory).await?);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let mut app = Router::new()
        .route("/", post(handle_proxy))
        .route("/health", get(health))
        .with_state(state.clone())
        .layer(cors);

    if let Some(token) = auth_token.as_ref() {
        info!("authentication token required for proxy requests");
        let token = Arc::new(token.clone());
        let auth_layer = from_fn(move |req, next| {
            let token = Arc::clone(&token);
            async move { enforce_bearer(token, req, next).await }
        });
        app = app.layer(auth_layer);
    } else if !listen.ip().is_loopback() {
        warn!(
            "proxy server is listening on {listen} without authentication; set GLUES_SERVER_TOKEN or --auth-token to protect access"
        );
    }

    let listener = TcpListener::bind(listen).await?;
    info!("listening on {}", listen);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub async fn run_cli() -> Result<()> {
    run(parse_args()).await
}

async fn build_backend(storage: StorageCommand) -> Result<Box<dyn CoreBackend + Send>> {
    let backend: Box<dyn CoreBackend + Send> = match storage {
        StorageCommand::Memory => Box::new(Db::memory().await?),
        StorageCommand::File { path } => Box::new(Db::file(&path).await?),
        StorageCommand::Redb { path } => Box::new(Db::redb(&path).await?),
        StorageCommand::Git {
            path,
            remote,
            branch,
        } => Box::new(Db::git(&path, remote, branch).await?),
        StorageCommand::Mongo { conn_str, db_name } => {
            Box::new(Db::mongo(&conn_str, &db_name).await?)
        }
    };

    Ok(backend)
}

async fn handle_proxy(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<ProxyRequest>,
) -> (StatusCode, Json<ProxyResponse>) {
    let response = state.handle(request).await;
    (StatusCode::OK, Json(response))
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn shutdown_signal() {
    if let Err(err) = signal::ctrl_c().await {
        error!("failed to install Ctrl+C handler: {err}");
        return;
    }

    info!("shutting down");
}

async fn enforce_bearer(
    token: Arc<String>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let Some(header) = req.headers().get(AUTHORIZATION) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Ok(value) = header.to_str() else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Some(provided) = value.strip_prefix("Bearer ").map(str::trim) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if provided != token.as_str() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use tower::ServiceExt;

    async fn ok() -> StatusCode {
        StatusCode::OK
    }

    #[tokio::test]
    async fn options_requests_bypass_auth() {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let token = Arc::new("secret".to_owned());
        let app = Router::new()
            .route("/", get(ok))
            .layer(cors)
            .layer(from_fn(move |req, next| {
                let token = Arc::clone(&token);
                async move { enforce_bearer(token, req, next).await }
            }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("preflight request should succeed");

        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn missing_token_still_rejected_for_non_preflight() {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let token = Arc::new("secret".to_owned());
        let app = Router::new()
            .route("/", get(ok))
            .layer(cors)
            .layer(from_fn(move |req, next| {
                let token = Arc::clone(&token);
                async move { enforce_bearer(token, req, next).await }
            }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
