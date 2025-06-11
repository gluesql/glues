use glues_core::{
    db::Db,
    proxy::{request::ProxyRequest, ProxyServer},
};
use std::sync::{mpsc::channel, Arc};
use tiny_http::{Response, Server};
use tokio::sync::Mutex;

fn main() {
    // Initialize a dedicated runtime for async operations
    let rt = tokio::runtime::Runtime::new().expect("create runtime");

    let (tx, _rx) = channel();
    let db = rt
        .block_on(Db::memory(tx))
        .expect("init db");
    let server = Arc::new(Mutex::new(ProxyServer::new(db)));

    let http = Server::http("0.0.0.0:9001").unwrap();
    for mut req in http.incoming_requests() {
        let mut body = String::new();
        req.as_reader().read_to_string(&mut body).unwrap();
        let proxy_req: ProxyRequest = serde_json::from_str(&body).unwrap();
        let srv = server.clone();
        let response = rt.block_on(async {
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
