use glues_core::state::GetInner;
use glues_core::{
    EntryEvent, Glues, KeyEvent, NotebookEvent, Transition,
    state::NotebookState,
    transition::{NormalModeTransition, NoteTreeTransition, NotebookTransition},
};

macro_rules! dispatch {
    ($glues:expr => $event:expr) => {
        $glues.dispatch($event.into()).await.unwrap()
    };
    ($glues:expr => $event:expr => $pattern:pat $(if $guard:expr)? $(,)?) => {{
        let t = $glues.dispatch($event.into()).await.unwrap();
        assert!(matches!(t, $pattern $(if $guard)?));
        t
    }};
}
#[tokio::test]
async fn memory_state_machine_basic() {
    let mut glues = Glues::new().await;

    dispatch!(glues => EntryEvent::OpenMemory =>
        Transition::Entry(glues_core::EntryTransition::OpenNotebook));

    // Verify sample note exists
    let state: &NotebookState = glues.state.get_inner().unwrap();
    let root_children = state.root.children.as_ref().unwrap();
    assert_eq!(root_children.notes.len(), 1);
    let sample_note = root_children.notes[0].clone();
    assert_eq!(sample_note.name, "Sample Note");

    // Select the note
    dispatch!(glues => NotebookEvent::SelectNote(sample_note.clone()) =>
        Transition::Notebook(NotebookTransition::None));

    // Open the note
    let Transition::Notebook(NotebookTransition::NoteTree(NoteTreeTransition::OpenNote {
        note,
        content,
    })) = dispatch!(glues => NotebookEvent::OpenNote)
    else {
        panic!("unexpected transition")
    };
    assert_eq!(note.id, sample_note.id);
    assert_eq!(content, "Hi :D");

    // Update note content
    let new_content = "Hello world".to_owned();
    dispatch!(glues => NotebookEvent::UpdateNoteContent {
        note_id: sample_note.id.clone(),
        content: new_content.clone(),
    } => Transition::Notebook(NotebookTransition::UpdateNoteContent(ref id)) if *id == sample_note.id);
    let db = glues.db.as_mut().unwrap();
    let fetched = db.fetch_note_content(sample_note.id.clone()).await.unwrap();
    assert_eq!(fetched, new_content);

    // Enter insert mode and then view note
    dispatch!(glues => KeyEvent::I => Transition::Notebook(NotebookTransition::EditingNormalMode(
        NormalModeTransition::InsertAtCursor
    )));

    let Transition::Notebook(NotebookTransition::ViewMode(note)) =
        dispatch!(glues => NotebookEvent::ViewNote)
    else {
        panic!("unexpected transition")
    };
    assert_eq!(note.id, sample_note.id);
}
