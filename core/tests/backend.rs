use glues_core::backend::{CoreBackend, local::Db};
use std::sync::mpsc::channel;

#[tokio::test]
async fn memory_backend_operations() {
    let (tx, _rx) = channel();
    let mut db = Db::memory(tx).await.unwrap();

    let root_id = db.root_id();
    let root = db.fetch_directory(root_id.clone()).await.unwrap();
    assert_eq!(root.name, "Notes");

    // add directory
    let dir = db
        .add_directory(root_id.clone(), "Work".to_owned())
        .await
        .unwrap();
    assert_eq!(dir.name, "Work");
    assert_eq!(dir.order, 0);

    let dir2 = db
        .add_directory(root_id.clone(), "Life".to_owned())
        .await
        .unwrap();
    assert_eq!(dir2.order, 1);

    // fetch directories
    let dirs = db.fetch_directories(root_id.clone()).await.unwrap();
    assert_eq!(dirs.len(), 2);
    assert_eq!(dirs[0].name, "Work");
    assert_eq!(dirs[1].name, "Life");

    // add note
    let note = db
        .add_note(dir.id.clone(), "Todo".to_owned())
        .await
        .unwrap();
    assert_eq!(note.name, "Todo");
    assert_eq!(note.order, 0);

    let note2 = db
        .add_note(dir.id.clone(), "Plan".to_owned())
        .await
        .unwrap();
    assert_eq!(note2.order, 1);

    // fetch notes
    let notes = db.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes.len(), 2);
    assert_eq!(notes[0].name, "Todo");
    assert_eq!(notes[1].name, "Plan");

    // update note content
    db.update_note_content(note.id.clone(), "hello".to_owned())
        .await
        .unwrap();
    let content = db.fetch_note_content(note.id.clone()).await.unwrap();
    assert_eq!(content, "hello");

    // rename note
    db.rename_note(note.id.clone(), "Hello".to_owned())
        .await
        .unwrap();
    let notes = db.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes[0].name, "Hello");
    assert_eq!(notes[1].name, "Plan");

    // move note to root
    db.move_note(note.id.clone(), root_id.clone())
        .await
        .unwrap();
    let notes_root = db.fetch_notes(root_id.clone()).await.unwrap();
    assert_eq!(notes_root.len(), 1);
    assert_eq!(notes_root[0].name, "Hello");

    // remove note
    db.remove_note(note.id.clone()).await.unwrap();
    let notes_root = db.fetch_notes(root_id.clone()).await.unwrap();
    assert!(notes_root.is_empty());

    // remove directory
    db.remove_directory(dir.id.clone()).await.unwrap();
    db.remove_directory(dir2.id.clone()).await.unwrap();
    let dirs = db.fetch_directories(root_id.clone()).await.unwrap();
    assert!(dirs.is_empty());

    // logging
    db.log("test".to_owned(), "message".to_owned())
        .await
        .unwrap();
}
