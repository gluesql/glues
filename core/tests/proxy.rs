use glues_core::backend::{
    CoreBackend,
    local::Db,
    proxy::{ProxyClient, ProxyServer, request::ProxyRequest},
};
use std::{
    net::TcpListener,
    sync::{Arc, mpsc::channel},
};
use tiny_http::{Response, Server};
use tokio::sync::Mutex;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test(flavor = "current_thread")]
async fn proxy_backend_operations() {
    let (tx, _rx) = channel();
    let db = Db::memory(tx).await.unwrap();
    let server = ProxyServer::new(Box::new(db));
    let server = Arc::new(Mutex::new(server));

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let http = Arc::new(Server::from_listener(listener, None).unwrap());
    let handle = tokio::runtime::Handle::current();
    let srv = server.clone();
    let http_clone = http.clone();
    let server_thread = std::thread::spawn(move || {
        for mut req in http_clone.incoming_requests() {
            let mut body = String::new();
            req.as_reader().read_to_string(&mut body).unwrap();
            let proxy_req: ProxyRequest = serde_json::from_str(&body).unwrap();
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
    });

    let mut client = ProxyClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let root_id = client.root_id();
    let root = client.fetch_directory(root_id.clone()).await.unwrap();
    assert_eq!(root.name, "Notes");

    let dir = client
        .add_directory(root_id.clone(), "Work".to_owned())
        .await
        .unwrap();
    assert_eq!(dir.name, "Work");

    let dirs = client.fetch_directories(root_id.clone()).await.unwrap();
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].name, "Work");

    let note = client
        .add_note(dir.id.clone(), "Todo".to_owned())
        .await
        .unwrap();
    assert_eq!(note.name, "Todo");

    let notes = client.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].name, "Todo");

    client
        .update_note_content(note.id.clone(), "hello".to_owned())
        .await
        .unwrap();
    let content = client.fetch_note_content(note.id.clone()).await.unwrap();
    assert_eq!(content, "hello");

    client
        .rename_note(note.id.clone(), "Hello".to_owned())
        .await
        .unwrap();
    let notes = client.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes[0].name, "Hello");

    client
        .move_note(note.id.clone(), root_id.clone())
        .await
        .unwrap();
    let notes_root = client.fetch_notes(root_id.clone()).await.unwrap();
    assert_eq!(notes_root.len(), 1);

    client.remove_note(note.id.clone()).await.unwrap();
    let notes_root = client.fetch_notes(root_id.clone()).await.unwrap();
    assert!(notes_root.is_empty());

    client.remove_directory(dir.id.clone()).await.unwrap();
    let dirs = client.fetch_directories(root_id.clone()).await.unwrap();
    assert!(dirs.is_empty());

    client
        .log("test".to_owned(), "message".to_owned())
        .await
        .unwrap();

    http.unblock();
    server_thread.join().unwrap();
}
