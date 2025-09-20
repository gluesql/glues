use {
    axum::{
        Json, Router,
        extract::State,
        http::StatusCode,
        routing::{get, post},
    },
    clap::{Args, Parser, Subcommand},
    color_eyre::Result,
    glues_core::{
        Task, Transition,
        backend::{
            CoreBackend,
            local::Db,
            proxy::{ProxyServer, request::ProxyRequest, response::ProxyResponse},
        },
        handle_tasks,
    },
    std::{
        collections::VecDeque,
        net::SocketAddr,
        sync::{
            Arc, Mutex,
            mpsc::{Sender, channel},
        },
        time::Duration,
    },
    tokio::{net::TcpListener, signal, sync::Mutex as AsyncMutex, time::sleep},
    tower_http::cors::{Any, CorsLayer},
    tracing::{error, info},
    tracing_subscriber::EnvFilter,
};

#[derive(Clone, Args)]
pub struct ServerArgs {
    #[arg(long, default_value = "127.0.0.1:4000")]
    pub listen: SocketAddr,

    #[command(subcommand)]
    pub storage: StorageCommand,
}

#[derive(Parser)]
#[command(author, version, about = "Glues proxy server")]
struct Cli {
    #[command(flatten)]
    args: ServerArgs,
}

#[derive(Subcommand, Clone)]
pub enum StorageCommand {
    /// In-memory storage (data resets on restart)
    Memory,
    /// File storage backend rooted at the given path
    File { path: String },
    /// Git storage backend
    Git {
        path: String,
        remote: String,
        branch: String,
    },
    /// MongoDB storage backend
    Mongo { conn_str: String, db_name: String },
}

pub fn parse_args() -> ServerArgs {
    Cli::parse().args
}

pub async fn run(args: ServerArgs) -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_target(false)
        .init();

    let (task_tx, task_rx) = channel();
    let transition_queue = Arc::new(Mutex::new(VecDeque::<Transition>::new()));
    let _task_handle = handle_tasks(task_rx, &transition_queue);
    spawn_transition_drain(Arc::clone(&transition_queue));

    let backend = build_backend(args.storage, task_tx).await?;
    let server = Arc::new(AsyncMutex::new(ProxyServer::new(backend)));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", post(handle_proxy))
        .route("/health", get(health))
        .with_state(server.clone())
        .layer(cors);

    let listener = TcpListener::bind(args.listen).await?;
    info!("listening on {}", args.listen);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub async fn run_cli() -> Result<()> {
    run(parse_args()).await
}

async fn build_backend(
    storage: StorageCommand,
    task_tx: Sender<Task>,
) -> Result<Box<dyn CoreBackend + Send>> {
    let backend: Box<dyn CoreBackend + Send> = match storage {
        StorageCommand::Memory => Box::new(Db::memory(task_tx.clone()).await?),
        StorageCommand::File { path } => Box::new(Db::file(task_tx.clone(), &path).await?),
        StorageCommand::Git {
            path,
            remote,
            branch,
        } => Box::new(Db::git(task_tx.clone(), &path, remote, branch).await?),
        StorageCommand::Mongo { conn_str, db_name } => {
            Box::new(Db::mongo(task_tx, &conn_str, &db_name).await?)
        }
    };

    Ok(backend)
}

async fn handle_proxy(
    State(server): State<Arc<AsyncMutex<ProxyServer>>>,
    Json(request): Json<ProxyRequest>,
) -> (StatusCode, Json<ProxyResponse>) {
    let mut server = server.lock_owned().await;
    let response = server.handle(request).await;
    (StatusCode::OK, Json(response))
}

async fn health() -> StatusCode {
    StatusCode::OK
}

fn spawn_transition_drain(queue: Arc<Mutex<VecDeque<Transition>>>) {
    tokio::spawn(async move {
        loop {
            {
                let mut guard = queue.lock().expect("transition queue poisoned");
                guard.clear();
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
}

async fn shutdown_signal() {
    if let Err(err) = signal::ctrl_c().await {
        error!("failed to install Ctrl+C handler: {err}");
        return;
    }

    info!("shutting down");
}
