use {
    glues_core::backend::{
        local::Db,
        proxy::{
            request::ProxyRequest,
            response::{ProxyResponse, ResultPayload},
        },
    },
    glues_server::state::ServerState,
};

fn expect_directory(response: ProxyResponse) -> glues_core::data::Directory {
    match response {
        ProxyResponse::Ok(ResultPayload::Directory(dir)) => dir,
        other => panic!("expected directory response, got {other:?}"),
    }
}

fn expect_note(response: ProxyResponse) -> glues_core::data::Note {
    match response {
        ProxyResponse::Ok(ResultPayload::Note(note)) => note,
        other => panic!("expected note response, got {other:?}"),
    }
}

fn expect_id(response: ProxyResponse) -> glues_core::types::DirectoryId {
    match response {
        ProxyResponse::Ok(ResultPayload::Id(id)) => id,
        other => panic!("expected id response, got {other:?}"),
    }
}

fn expect_unit(response: ProxyResponse) {
    match response {
        ProxyResponse::Ok(ResultPayload::Unit) => {}
        other => panic!("expected unit response, got {other:?}"),
    }
}

fn expect_err_contains(response: ProxyResponse, needle: &str) {
    match response {
        ProxyResponse::Err(message) if message.contains(needle) => {}
        ProxyResponse::Err(message) => panic!("unexpected error message: {message}"),
        other => panic!("expected error response, got {other:?}"),
    }
}

#[tokio::test]
async fn proxy_guard_enforces_allowed_subtree() {
    let db = Db::memory().await.expect("memory backend");
    let mut proxy = glues_core::backend::proxy::ProxyServer::new(Box::new(db));
    let root_id = proxy.db.root_id();

    let allowed_dir = expect_directory(
        proxy
            .handle(ProxyRequest::AddDirectory {
                parent_id: root_id.clone(),
                name: "allowed".into(),
            })
            .await,
    );

    let outside_dir = expect_directory(
        proxy
            .handle(ProxyRequest::AddDirectory {
                parent_id: root_id.clone(),
                name: "outside".into(),
            })
            .await,
    );

    let nested_dir = expect_directory(
        proxy
            .handle(ProxyRequest::AddDirectory {
                parent_id: allowed_dir.id.clone(),
                name: "nested".into(),
            })
            .await,
    );

    let note = expect_note(
        proxy
            .handle(ProxyRequest::AddNote {
                directory_id: nested_dir.id.clone(),
                name: "note".into(),
            })
            .await,
    );

    let state = ServerState::new(proxy, Some(allowed_dir.id.clone()))
        .await
        .expect("create server state");

    let root_response = state.handle(ProxyRequest::RootId).await;
    let seen_root = expect_id(root_response);
    assert_eq!(seen_root, allowed_dir.id);

    expect_err_contains(
        state
            .handle(ProxyRequest::FetchDirectory {
                directory_id: outside_dir.id.clone(),
            })
            .await,
        "outside the allowed subtree",
    );

    expect_err_contains(
        state
            .handle(ProxyRequest::AddDirectory {
                parent_id: root_id.clone(),
                name: "blocked".into(),
            })
            .await,
        "outside the allowed subtree",
    );

    let new_child = expect_directory(
        state
            .handle(ProxyRequest::AddDirectory {
                parent_id: allowed_dir.id.clone(),
                name: "child".into(),
            })
            .await,
    );

    expect_unit(
        state
            .handle(ProxyRequest::RenameDirectory {
                directory_id: new_child.id.clone(),
                name: "renamed".into(),
            })
            .await,
    );

    expect_err_contains(
        state
            .handle(ProxyRequest::MoveNote {
                note_id: note.id.clone(),
                directory_id: outside_dir.id.clone(),
            })
            .await,
        "outside the allowed subtree",
    );

    expect_unit(
        state
            .handle(ProxyRequest::RemoveDirectory {
                directory_id: nested_dir.id.clone(),
            })
            .await,
    );

    expect_err_contains(
        state
            .handle(ProxyRequest::FetchNoteContent {
                note_id: note.id.clone(),
            })
            .await,
        "outside the allowed subtree",
    );

    expect_err_contains(
        state
            .handle(ProxyRequest::RenameDirectory {
                directory_id: nested_dir.id.clone(),
                name: "should-fail".into(),
            })
            .await,
        "outside the allowed subtree",
    );

    expect_err_contains(
        state
            .handle(ProxyRequest::RenameDirectory {
                directory_id: allowed_dir.id.clone(),
                name: "forbidden".into(),
            })
            .await,
        "not permitted",
    );
}
