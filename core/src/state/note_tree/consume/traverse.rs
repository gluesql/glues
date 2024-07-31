use crate::{
    state::note_tree::{InnerState, NoteTreeState, SelectedItem, TreeItem},
    Error, Result, Transition,
};

pub fn select_next(state: &mut NoteTreeState) -> Result<Transition> {
    select(state, true)
}

pub fn select_prev(state: &mut NoteTreeState) -> Result<Transition> {
    select(state, false)
}

pub fn select(state: &mut NoteTreeState, next: bool) -> Result<Transition> {
    let id = match &state.selected {
        SelectedItem::Note(note) => note.id.clone(),
        SelectedItem::Directory(directory) => directory.id.clone(),
        SelectedItem::None => return Err(Error::Wip("selected item not found".to_owned())),
    };

    let tree_item = if next {
        state.root.find_next(&id)
    } else {
        state.root.find_prev(&id)
    };

    Ok(match tree_item {
        Some(TreeItem::Note(note)) => {
            state.selected = SelectedItem::Note(note.clone());
            state.inner_state = InnerState::NoteSelected;

            Transition::SelectNote(note.clone())
        }
        Some(TreeItem::Directory(directory)) => {
            state.selected = SelectedItem::Directory(directory.clone());
            state.inner_state = InnerState::DirectorySelected;

            Transition::SelectDirectory(directory.clone())
        }
        None => Transition::None,
    })
}
