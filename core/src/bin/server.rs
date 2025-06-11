use clap::{Parser, Subcommand};
use glues_core::{
    Task,
    db::Db,
    handle_tasks,
    proxy::{ProxyServer, request::ProxyRequest},
    transition::Transition,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, mpsc::channel},
};
use tiny_http::{Header, Response, Server};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Parser)]
#[command(author, version, about = "Glues HTTP server")]
struct Cli {
    #[arg(long, default_value_t = 3000, global = true)]
    port: u16,
    #[command(subcommand)]
    storage: Storage,
}

#[derive(Subcommand)]
enum Storage {
    Memory,
    Csv {
        path: String,
    },
    Json {
        path: String,
    },
    File {
        path: String,
    },
    Git {
        path: String,
        remote: String,
        branch: String,
    },
    Mongo {
        conn_str: String,
        db_name: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let transition_queue = Arc::new(Mutex::new(VecDeque::<Transition>::new()));
    let (task_tx, task_rx) = channel::<Task>();
    let _handle = handle_tasks(task_rx, &transition_queue);

    let db = match cli.storage {
        Storage::Memory => Db::memory(task_tx).await?,
        Storage::Csv { path } => Db::csv(task_tx, &path).await?,
        Storage::Json { path } => Db::json(task_tx, &path).await?,
        Storage::File { path } => Db::file(task_tx, &path).await?,
        Storage::Git {
            path,
            remote,
            branch,
        } => Db::git(task_tx, &path, remote, branch).await?,
        Storage::Mongo { conn_str, db_name } => Db::mongo(task_tx, &conn_str, &db_name).await?,
    };

    let server = ProxyServer::new(db);
    let server = Arc::new(AsyncMutex::new(server));

    let addr = format!("0.0.0.0:{}", cli.port);
    let http = Server::http(&addr)?;
    println!("Listening on http://{}", addr);

    let rt_handle = tokio::runtime::Handle::current();
    for mut req in http.incoming_requests() {
        let srv = server.clone();
        let handle = rt_handle.clone();
        tokio::task::spawn_blocking(move || {
            let mut body = String::new();
            if req.as_reader().read_to_string(&mut body).is_err() {
                return;
            }
            let proxy_req: ProxyRequest = match serde_json::from_str(&body) {
                Ok(req) => req,
                Err(_) => {
                    let _ = req.respond(Response::from_string("bad request"));
                    return;
                }
            };

            let response = handle.block_on(async {
                let mut s = srv.lock().await;
                s.handle(proxy_req).await
            });

            let body = match serde_json::to_string(&response) {
                Ok(b) => b,
                Err(_) => return,
            };
            let resp = Response::from_string(body)
                .with_header(Header::from_bytes("Content-Type", "application/json").unwrap());
            let _ = req.respond(resp);
        });
    }

    Ok(())
}
