use crate::{data::Note, state::note_tree::DirectoryItem, Error, Result};

pub enum Transition<'a> {
    Initialize,

    OpenDirectory(OpenDirectory<'a>),
    CloseDirectory,
    /*
    AddNote {
        directory_id: DirectoryId,
        name: String,
    },
    AddDirectory,

    RemoveNote,
    RemoveDirectory,

    RenameNote,
    RenameDirectory,
    */
}

pub struct OpenDirectory<'a> {
    pub notes: &'a [Note],
    pub directories: &'a [DirectoryItem],
}

pub trait GetTransition<T> {
    fn get_transition(self) -> Result<T>;
}

impl<'a> GetTransition<()> for Transition<'a> {
    fn get_transition(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> GetTransition<OpenDirectory<'a>> for Transition<'a> {
    fn get_transition(self) -> Result<OpenDirectory<'a>> {
        match self {
            Self::OpenDirectory(v) => Ok(v),
            _ => Err(Error::Wip(
                "Transition::get_transition for transition::OpenDirectory failed".to_owned(),
            )),
        }
    }
}

impl<'a> From<OpenDirectory<'a>> for Transition<'a> {
    fn from(v: OpenDirectory<'a>) -> Self {
        Self::OpenDirectory(v)
    }
}
