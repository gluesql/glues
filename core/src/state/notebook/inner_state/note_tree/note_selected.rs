use {
    super::NoteTreeState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
        backend::CoreBackend,
        state::notebook::{InnerState, NotebookState, directory, note, tabs},
        transition::{MoveModeTransition, NoteTreeTransition},
        types::{KeymapGroup, KeymapItem},
    },
};

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Notebook(OpenDirectory(directory_id)) => directory::open(db, state, directory_id).await,
        Notebook(CloseDirectory(directory_id)) => {
            let directory = state
                .root
                .find(&directory_id)
                .ok_or(Error::NotFound(
                    "[CloseDirectory] failed to find target directory".to_owned(),
                ))?
                .directory
                .clone();

            directory::close(state, directory)
        }
        Key(KeyEvent::H) | Key(KeyEvent::Left) => {
            let directory_id = &state.get_selected_note()?.directory_id;
            let directory_item = state.root.find(directory_id).ok_or(Error::NotFound(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let directory = directory_item.directory.clone();

            directory::close(state, directory)
        }
        Key(KeyEvent::J | KeyEvent::Down) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectNext(1),
        )),
        Key(KeyEvent::K | KeyEvent::Up) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectPrev(1),
        )),
        Key(KeyEvent::CapJ) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectNextDirectory,
        )),
        Key(KeyEvent::CapK) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectPrevDirectory,
        )),
        Key(KeyEvent::M) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        Key(KeyEvent::Space) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::MoveMode);

            Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
                MoveModeTransition::Enter,
            )))
        }
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::L | KeyEvent::Enter) | Notebook(OpenNote) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::Numbering(n.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::CapG) => Ok(NotebookTransition::NoteTree(NoteTreeTransition::SelectLast)),
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::GatewayMode);

            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::GatewayMode,
            ))
        }
        Key(KeyEvent::AngleBracketOpen) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::ShrinkWidth(1),
        )),
        Key(KeyEvent::AngleBracketClose) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::ExpandWidth(1),
        )),
        Key(KeyEvent::Tab) if !state.tabs.is_empty() => tabs::focus_editor(db, state).await,
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Todo(
            "Notebook::NoteTree::NoteSelected::consume".to_owned(),
        )),
    }
}

pub fn keymap(state: &NotebookState) -> Vec<KeymapGroup> {
    let navigation = vec![
        KeymapItem::new("j", "Select next"),
        KeymapItem::new("k", "Select previous"),
        KeymapItem::new("J", "Select next directory"),
        KeymapItem::new("K", "Select parent directory"),
        KeymapItem::new("G", "Select last"),
        KeymapItem::new("1-9", "Add steps"),
        KeymapItem::new(">", "Expand width"),
        KeymapItem::new("<", "Shrink width"),
    ];

    let mut actions = vec![
        KeymapItem::new("l", "Open note"),
        KeymapItem::new("h", "Close parent directory"),
        KeymapItem::new("g", "Enter gateway mode"),
        KeymapItem::new("Space", "Move note"),
        KeymapItem::new("m", "Show more actions"),
    ];

    if !state.tabs.is_empty() {
        actions.push(KeymapItem::new("Tab", "Focus editor"));
    }

    actions.push(KeymapItem::new("Esc", "Quit"));

    vec![
        KeymapGroup::new("Navigation", navigation),
        KeymapGroup::new("Actions", actions),
    ]
}
