use {
    super::NoteTreeState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
        db::CoreBackend,
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
        Key(KeyEvent::L | KeyEvent::Right | KeyEvent::Enter) => {
            let directory = state.get_selected_directory()?.clone();
            let directory_item = state.root.find(&directory.id).ok_or(Error::Wip(
                "[Key::L] failed to find the target directory".to_owned(),
            ))?;

            if directory_item.children.is_none() {
                directory::open(db, state, directory.id.clone()).await
            } else {
                directory::close(state, directory)
            }
        }
        Notebook(CloseDirectory(directory_id)) => {
            let directory = state
                .root
                .find(&directory_id)
                .ok_or(Error::Wip(
                    "[CloseDirectory] failed to find target directory".to_owned(),
                ))?
                .directory
                .clone();

            directory::close(state, directory)
        }
        Key(KeyEvent::H) | Key(KeyEvent::Left) => {
            let directory = state.get_selected_directory()?;
            if state.root.directory.id == directory.id {
                return Ok(NotebookTransition::None);
            }

            let parent_item = state.root.find(&directory.parent_id).ok_or(Error::Wip(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let parent = parent_item.directory.clone();

            directory::close(state, parent)
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
            let directory = state.get_selected_directory()?.clone();

            directory::show_actions_dialog(state, directory)
        }
        Key(KeyEvent::Space) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::MoveMode);

            Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
                MoveModeTransition::Enter,
            )))
        }
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
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
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

pub fn keymap(state: &NotebookState) -> Vec<KeymapGroup> {
    let mut items = vec![
        KeymapItem::new("l", "Toggle directory"),
        KeymapItem::new("h", "Close parent directory"),
        KeymapItem::new("j", "Select next"),
        KeymapItem::new("k", "Select previous"),
        KeymapItem::new("J", "Select next directory"),
        KeymapItem::new("K", "Select previous directory"),
        KeymapItem::new("G", "Select last"),
        KeymapItem::new("1-9", "Add steps"),
        KeymapItem::new(">", "Expand width"),
        KeymapItem::new("<", "Shrink width"),
        KeymapItem::new("Space", "Move directory"),
        KeymapItem::new("m", "Show more actions"),
    ];

    if !state.tabs.is_empty() {
        items.push(KeymapItem::new("Tab", "Focus editor"));
    }

    items.push(KeymapItem::new("Esc", "Quit"));

    vec![KeymapGroup::new("General", items)]
}
