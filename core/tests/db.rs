use glues_core::db::Db;
use std::sync::mpsc::channel;

#[tokio::test]
async fn memory_db_operations() {
    let (tx, _rx) = channel();
    let mut db = Db::memory(tx).await.unwrap();

    let root_id = db.root_id.clone();
    let root = db.fetch_directory(root_id.clone()).await.unwrap();
    assert_eq!(root.name, "Notes");

    let dir = db
        .add_directory(root_id.clone(), "Work".to_owned())
        .await
        .unwrap();
    assert_eq!(dir.name, "Work");

    let dirs = db.fetch_directories(root_id.clone()).await.unwrap();
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].name, "Work");

    let note = db
        .add_note(dir.id.clone(), "Todo".to_owned())
        .await
        .unwrap();
    assert_eq!(note.name, "Todo");

    let notes = db.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].name, "Todo");

    db.update_note_content(note.id.clone(), "hello".to_owned())
        .await
        .unwrap();
    let content = db.fetch_note_content(note.id.clone()).await.unwrap();
    assert_eq!(content, "hello");

    db.rename_note(note.id.clone(), "Hello".to_owned())
        .await
        .unwrap();
    let notes = db.fetch_notes(dir.id.clone()).await.unwrap();
    assert_eq!(notes[0].name, "Hello");

    db.move_note(note.id.clone(), root_id.clone())
        .await
        .unwrap();
    let notes_root = db.fetch_notes(root_id.clone()).await.unwrap();
    assert_eq!(notes_root.len(), 1);

    db.remove_note(note.id.clone()).await.unwrap();
    let notes_root = db.fetch_notes(root_id.clone()).await.unwrap();
    assert!(notes_root.is_empty());

    db.remove_directory(dir.id.clone()).await.unwrap();
    let dirs = db.fetch_directories(root_id.clone()).await.unwrap();
    assert!(dirs.is_empty());

    db.log("test".to_owned(), "message".to_owned())
        .await
        .unwrap();

    db.pull().await.unwrap();
    db.sync().unwrap();
}
