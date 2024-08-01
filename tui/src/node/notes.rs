mod editor;
mod note_tree;

use {editor::EditorNode, note_tree::NoteTreeNode};

pub struct NotesNode;

impl NotesNode {
    pub fn note_tree(&self) -> NoteTreeNode {
        NoteTreeNode
    }

    pub fn editor(&self) -> EditorNode {
        EditorNode
    }
}
