use {
    color_eyre::Result,
    glues_core::{
        backend::proxy::{
            ProxyServer,
            request::ProxyRequest,
            response::{ProxyResponse, ResultPayload},
        },
        types::{DirectoryId, NoteId},
    },
    std::{
        collections::{HashSet, VecDeque},
        sync::Arc,
    },
    tokio::sync::RwLock,
};

pub(crate) struct ProxyAccess {
    pub(crate) allowed_root: DirectoryId,
    pub(crate) directories: HashSet<DirectoryId>,
    pub(crate) notes: HashSet<NoteId>,
}

impl ProxyAccess {
    pub(crate) fn new(
        allowed_root: DirectoryId,
        directories: HashSet<DirectoryId>,
        notes: HashSet<NoteId>,
    ) -> Self {
        Self {
            allowed_root,
            directories,
            notes,
        }
    }

    pub(crate) fn evaluate(&self, request: &ProxyRequest) -> GuardEvaluation {
        use ProxyRequest::*;

        match request {
            RootId => GuardEvaluation::ReturnRoot {
                root: self.allowed_root.clone(),
            },
            FetchDirectory { directory_id } => {
                self.guard_directory(directory_id, GuardEvaluation::allow())
            }
            FetchDirectories { parent_id }
            | FetchNotes {
                directory_id: parent_id,
            } => self.guard_directory(parent_id, GuardEvaluation::allow()),
            FetchNoteContent { note_id } => self.guard_note(note_id, GuardEvaluation::allow()),
            AddDirectory { parent_id, .. } => {
                self.guard_directory(parent_id, GuardEvaluation::allow())
            }
            RemoveDirectory { directory_id } => self.guard_directory(directory_id, {
                if directory_id == &self.allowed_root {
                    GuardEvaluation::Deny {
                        message: "proxy: modifying the allowed root directory is not permitted"
                            .to_owned(),
                    }
                } else {
                    GuardEvaluation::Allow {
                        pending: Some(PendingMutation::CollectRemoval {
                            directory_id: directory_id.clone(),
                        }),
                    }
                }
            }),
            MoveDirectory {
                directory_id,
                parent_id,
            } => {
                if directory_id == &self.allowed_root {
                    return GuardEvaluation::Deny {
                        message: "proxy: moving the allowed root directory is not permitted"
                            .to_owned(),
                    };
                }
                match (
                    self.directories.contains(directory_id),
                    self.directories.contains(parent_id),
                ) {
                    (true, true) => GuardEvaluation::allow(),
                    (false, _) => GuardEvaluation::deny_directory(directory_id),
                    (_, false) => GuardEvaluation::deny_directory(parent_id),
                }
            }
            RenameDirectory { directory_id, .. } => {
                if directory_id == &self.allowed_root {
                    GuardEvaluation::Deny {
                        message: "proxy: renaming the allowed root directory is not permitted"
                            .to_owned(),
                    }
                } else {
                    self.guard_directory(directory_id, GuardEvaluation::allow())
                }
            }
            AddNote { directory_id, .. } => {
                self.guard_directory(directory_id, GuardEvaluation::allow())
            }
            RemoveNote { note_id } => self.guard_note(note_id, GuardEvaluation::allow()),
            RenameNote { note_id, .. } | UpdateNoteContent { note_id, .. } => {
                self.guard_note(note_id, GuardEvaluation::allow())
            }
            MoveNote {
                note_id,
                directory_id,
            } => {
                let note_allowed = self.notes.contains(note_id);
                let dir_allowed = self.directories.contains(directory_id);
                match (note_allowed, dir_allowed) {
                    (true, true) => GuardEvaluation::allow(),
                    (false, _) => GuardEvaluation::deny_note(note_id),
                    (_, false) => GuardEvaluation::deny_directory(directory_id),
                }
            }
            Log { .. } | Sync => GuardEvaluation::allow(),
        }
    }

    fn guard_directory(&self, directory_id: &DirectoryId, ok: GuardEvaluation) -> GuardEvaluation {
        if self.directories.contains(directory_id) {
            ok
        } else {
            GuardEvaluation::deny_directory(directory_id)
        }
    }

    fn guard_note(&self, note_id: &NoteId, ok: GuardEvaluation) -> GuardEvaluation {
        if self.notes.contains(note_id) {
            ok
        } else {
            GuardEvaluation::deny_note(note_id)
        }
    }
}

pub(crate) enum GuardEvaluation {
    Allow { pending: Option<PendingMutation> },
    ReturnRoot { root: DirectoryId },
    Deny { message: String },
}

impl GuardEvaluation {
    pub(crate) fn allow() -> GuardEvaluation {
        GuardEvaluation::Allow { pending: None }
    }

    pub(crate) fn deny_directory(id: &DirectoryId) -> GuardEvaluation {
        GuardEvaluation::Deny {
            message: format!("proxy: directory {id} is outside the allowed subtree"),
        }
    }

    pub(crate) fn deny_note(id: &NoteId) -> GuardEvaluation {
        GuardEvaluation::Deny {
            message: format!("proxy: note {id} is outside the allowed subtree"),
        }
    }
}

pub(crate) enum PendingMutation {
    CollectRemoval { directory_id: DirectoryId },
}

pub(crate) struct RemovalPlan {
    pub(crate) directories: Vec<DirectoryId>,
    pub(crate) notes: Vec<NoteId>,
}

pub(crate) enum PostPlan {
    None,
    AddDirectory,
    AddNote,
    RemoveNote { note_id: NoteId },
    RemoveDirectory(RemovalPlan),
}

impl PostPlan {
    pub(crate) fn from_request(request: &ProxyRequest) -> Self {
        match request {
            ProxyRequest::AddDirectory { .. } => PostPlan::AddDirectory,
            ProxyRequest::AddNote { .. } => PostPlan::AddNote,
            ProxyRequest::RemoveNote { note_id } => PostPlan::RemoveNote {
                note_id: note_id.clone(),
            },
            _ => PostPlan::None,
        }
    }
}

pub(crate) async fn load_proxy_access(
    server: &mut ProxyServer,
    root: DirectoryId,
) -> Result<ProxyAccess> {
    // Ensure the root exists before building the cache.
    let _ = server.db.fetch_directory(root.clone()).await?;

    let mut directories = HashSet::new();
    let mut notes = HashSet::new();
    let mut queue = VecDeque::new();

    directories.insert(root.clone());
    queue.push_back(root.clone());

    while let Some(dir_id) = queue.pop_front() {
        let children = server.db.fetch_directories(dir_id.clone()).await?;
        for child in children {
            if directories.insert(child.id.clone()) {
                queue.push_back(child.id.clone());
            }
        }

        let dir_notes = server.db.fetch_notes(dir_id).await?;
        for note in dir_notes {
            notes.insert(note.id);
        }
    }

    Ok(ProxyAccess::new(root, directories, notes))
}

pub(crate) async fn collect_removal_plan(
    server: &mut ProxyServer,
    directory_id: DirectoryId,
) -> Result<RemovalPlan> {
    let mut directories = Vec::new();
    let mut notes = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(directory_id.clone());

    while let Some(dir_id) = queue.pop_front() {
        directories.push(dir_id.clone());

        let children = server.db.fetch_directories(dir_id.clone()).await?;
        for child in children {
            queue.push_back(child.id.clone());
        }

        let dir_notes = server.db.fetch_notes(dir_id).await?;
        for note in dir_notes {
            notes.push(note.id);
        }
    }

    Ok(RemovalPlan { directories, notes })
}
pub(crate) async fn apply_post_plan(
    access: &Arc<RwLock<ProxyAccess>>,
    plan: PostPlan,
    response: &ProxyResponse,
) {
    match plan {
        PostPlan::None => {}
        PostPlan::AddDirectory => {
            if let ProxyResponse::Ok(ResultPayload::Directory(directory)) = response {
                let mut guard = access.write().await;
                guard.directories.insert(directory.id.clone());
            }
        }
        PostPlan::AddNote => {
            if let ProxyResponse::Ok(ResultPayload::Note(note)) = response {
                let mut guard = access.write().await;
                guard.notes.insert(note.id.clone());
            }
        }
        PostPlan::RemoveNote { note_id } => {
            if matches!(response, ProxyResponse::Ok(ResultPayload::Unit)) {
                let mut guard = access.write().await;
                guard.notes.remove(&note_id);
            }
        }
        PostPlan::RemoveDirectory(RemovalPlan { directories, notes }) => {
            if matches!(response, ProxyResponse::Ok(ResultPayload::Unit)) {
                let mut guard = access.write().await;
                for directory_id in directories {
                    guard.directories.remove(&directory_id);
                }
                for note_id in notes {
                    guard.notes.remove(&note_id);
                }
            }
        }
    }
}
