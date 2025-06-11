use glues_core::{
    db::Db,
    proxy::{ProxyServer, request::ProxyRequest},
};
use std::io::Read;
use std::sync::{Arc, mpsc::channel};
use tiny_http::{Response, Server};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let (tx, _rx) = channel();
    let db = Db::memory(tx).await.expect("init db");
    let server = Arc::new(Mutex::new(ProxyServer::new(db)));

    let http = Server::http("0.0.0.0:9001").unwrap();
    let handle = tokio::runtime::Handle::current();
    for mut req in http.incoming_requests() {
        let mut body = String::new();
        req.as_reader().read_to_string(&mut body).unwrap();
        let proxy_req: ProxyRequest = serde_json::from_str(&body).unwrap();
        let srv = server.clone();
        let response = handle.block_on(async {
            let mut s = srv.lock().await;
            s.handle(proxy_req).await
        });
        let body = serde_json::to_string(&response).unwrap();
        let resp = Response::from_string(body).with_header(
            tiny_http::Header::from_bytes("Content-Type", "application/json").unwrap(),
        );
        let _ = req.respond(resp);
    }
}
