use glues_core::{
    Error,
    backend::{
        CoreBackend,
        local::Db,
        proxy::{ProxyClient, ProxyServer, request::ProxyRequest},
    },
};
use std::{
    io::ErrorKind,
    net::TcpListener,
    sync::{Arc, mpsc::channel},
};
use tiny_http::{Response, Server};
use tokio::sync::Mutex;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test(flavor = "current_thread")]
async fn proxy_backend_operations() {
    let (tx, _rx) = channel();
    let db = Db::memory(tx)
        .await
        .expect("in-memory proxy database should initialize");
    let server = ProxyServer::new(Box::new(db));
    let server = Arc::new(Mutex::new(server));

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("skipping proxy_backend_operations: {err}");
            return;
        }
        Err(err) => panic!("proxy server should bind to ephemeral port: {err}"),
    };
    let addr = listener
        .local_addr()
        .expect("proxy server should expose a local address");
    let http = Arc::new(
        Server::from_listener(listener, None).expect("proxy server should accept HTTP connections"),
    );
    let handle = tokio::runtime::Handle::current();
    let srv = server.clone();
    let http_clone = http.clone();
    let server_thread = std::thread::spawn(move || {
        for mut req in http_clone.incoming_requests() {
            let mut body = String::new();
            req.as_reader()
                .read_to_string(&mut body)
                .expect("proxy request body should read into string");
            let proxy_req: ProxyRequest =
                serde_json::from_str(&body).expect("proxy request JSON should deserialize");
            let response = handle.block_on(async {
                let mut s = srv.lock().await;
                s.handle(proxy_req).await
            });
            let body =
                serde_json::to_string(&response).expect("proxy response should serialize to JSON");
            let resp = Response::from_string(body).with_header(
                tiny_http::Header::from_bytes("Content-Type", "application/json")
                    .expect("content-type header should be valid"),
            );
            let _ = req.respond(resp);
        }
    });

    let mut client = ProxyClient::connect(format!("http://{addr}"), None)
        .await
        .expect("proxy client should connect to server");

    let root_id = client.root_id();
    let root = client
        .fetch_directory(root_id.clone())
        .await
        .expect("proxy client should fetch root directory");
    assert_eq!(root.name, "Notes");

    let dir = client
        .add_directory(root_id.clone(), "Work".to_owned())
        .await
        .expect("proxy client should add directory");
    assert_eq!(dir.name, "Work");

    let dirs = client
        .fetch_directories(root_id.clone())
        .await
        .expect("proxy client should list directories");
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].name, "Work");

    let note = client
        .add_note(dir.id.clone(), "Todo".to_owned())
        .await
        .expect("proxy client should add note");
    assert_eq!(note.name, "Todo");

    let notes = client
        .fetch_notes(dir.id.clone())
        .await
        .expect("proxy client should list notes in directory");
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].name, "Todo");

    client
        .update_note_content(note.id.clone(), "hello".to_owned())
        .await
        .expect("proxy client should update note content");
    let content = client
        .fetch_note_content(note.id.clone())
        .await
        .expect("proxy client should fetch note content");
    assert_eq!(content, "hello");

    client
        .rename_note(note.id.clone(), "Hello".to_owned())
        .await
        .expect("proxy client should rename note");
    let notes = client
        .fetch_notes(dir.id.clone())
        .await
        .expect("proxy client should list notes after rename");
    assert_eq!(notes[0].name, "Hello");

    client
        .move_note(note.id.clone(), root_id.clone())
        .await
        .expect("proxy client should move note");
    let notes_root = client
        .fetch_notes(root_id.clone())
        .await
        .expect("proxy client should list notes in root directory");
    assert_eq!(notes_root.len(), 1);

    client
        .remove_note(note.id.clone())
        .await
        .expect("proxy client should remove note");
    let notes_root = client
        .fetch_notes(root_id.clone())
        .await
        .expect("proxy client should list notes after removal");
    assert!(notes_root.is_empty());

    client
        .remove_directory(dir.id.clone())
        .await
        .expect("proxy client should remove directory");
    let dirs = client
        .fetch_directories(root_id.clone())
        .await
        .expect("proxy client should list directories after removal");
    assert!(dirs.is_empty());

    client
        .log("test".to_owned(), "message".to_owned())
        .await
        .expect("proxy client should append log entry");

    http.unblock();
    server_thread
        .join()
        .expect("proxy server thread should finish cleanly");
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test(flavor = "current_thread")]
async fn proxy_backend_requires_token() {
    let (tx, _rx) = channel();
    let db = Db::memory(tx)
        .await
        .expect("in-memory proxy database should initialize");
    let server = ProxyServer::new(Box::new(db));
    let server = Arc::new(Mutex::new(server));

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => listener,
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("skipping proxy_backend_requires_token: {err}");
            return;
        }
        Err(err) => panic!("proxy server should bind to ephemeral port: {err}"),
    };
    let addr = listener
        .local_addr()
        .expect("proxy server should expose a local address");
    let http = Arc::new(
        Server::from_listener(listener, None).expect("proxy server should accept HTTP connections"),
    );
    let handle = tokio::runtime::Handle::current();
    let srv = server.clone();
    let http_clone = http.clone();
    let token = "secret-token";
    let expected_header = format!("Bearer {token}");
    let server_thread = std::thread::spawn(move || {
        for mut req in http_clone.incoming_requests() {
            let header_ok = req
                .headers()
                .iter()
                .find(|h| h.field.equiv("Authorization"))
                .map(|h| h.value.as_str().trim() == expected_header)
                .unwrap_or(false);
            if !header_ok {
                let _ = req.respond(Response::empty(401));
                continue;
            }

            let mut body = String::new();
            req.as_reader()
                .read_to_string(&mut body)
                .expect("proxy request body should read into string");
            let proxy_req: ProxyRequest =
                serde_json::from_str(&body).expect("proxy request JSON should deserialize");
            let response = handle.block_on(async {
                let mut s = srv.lock().await;
                s.handle(proxy_req).await
            });
            let body =
                serde_json::to_string(&response).expect("proxy response should serialize to JSON");
            let resp = Response::from_string(body).with_header(
                tiny_http::Header::from_bytes("Content-Type", "application/json")
                    .expect("content-type header should be valid"),
            );
            let _ = req.respond(resp);
        }
    });

    let err = match ProxyClient::connect(format!("http://{addr}"), None).await {
        Ok(_) => panic!("proxy client should reject missing authentication token"),
        Err(err) => err,
    };
    match err {
        Error::Proxy(message) => {
            assert!(
                message.contains("authentication token"),
                "unexpected error message: {message}"
            );
        }
        other => panic!("unexpected error type: {other:?}"),
    }

    let mut client = ProxyClient::connect(format!("http://{addr}"), Some(token.to_owned()))
        .await
        .expect("proxy client should connect when token is provided");

    let root_id = client.root_id();
    let dirs = client
        .fetch_directories(root_id)
        .await
        .expect("proxy client should operate with authenticated session");
    assert!(dirs.is_empty());

    http.unblock();
    server_thread
        .join()
        .expect("proxy server thread should finish cleanly");
}
