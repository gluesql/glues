use glues_core::{data::Note, db::Db, proxy::ProxyServer};
use mcp::tools::{AddNote, GetNote, ListNotes, RootId};
use std::sync::{Arc, mpsc::channel};
use tokio::sync::Mutex;

#[tokio::test(flavor = "current_thread")]
async fn root_and_note_tools_flow() {
    let (tx, _rx) = channel();
    let db = Db::memory(tx).await.unwrap();
    let server = Arc::new(Mutex::new(ProxyServer::new(db)));

    // get root id
    let root_result = RootId {}.call_tool(server.clone()).await.unwrap();
    let root_id = root_result
        .content
        .first()
        .unwrap()
        .as_text_content()
        .unwrap()
        .text
        .clone();

    // add note
    let add = AddNote {
        directory_id: root_id.clone(),
        name: "Test".into(),
    };
    let add_result = add.call_tool(server.clone()).await.unwrap();
    let note: Note = serde_json::from_str(
        &add_result
            .content
            .first()
            .unwrap()
            .as_text_content()
            .unwrap()
            .text,
    )
    .unwrap();
    assert_eq!(note.name, "Test");

    // list notes
    let list = ListNotes {
        directory_id: root_id.clone(),
    };
    let list_result = list.call_tool(server.clone()).await.unwrap();
    let notes: Vec<Note> = serde_json::from_str(
        &list_result
            .content
            .first()
            .unwrap()
            .as_text_content()
            .unwrap()
            .text,
    )
    .unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].id, note.id);

    // get note content
    let get = GetNote {
        note_id: note.id.clone(),
    };
    let get_result = get.call_tool(server.clone()).await.unwrap();
    let content = get_result
        .content
        .first()
        .unwrap()
        .as_text_content()
        .unwrap()
        .text
        .clone();
    assert!(content.is_empty());
}
