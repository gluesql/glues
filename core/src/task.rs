use {
    crate::{Result, Transition},
    gluesql::gluesql_git_storage::{GitStorage, StorageType},
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::{mpsc::Receiver, Arc, Mutex},
        thread::{spawn, JoinHandle},
    },
};

#[derive(Clone, Debug)]
pub enum Task {
    GitSync {
        path: PathBuf,
        remote: String,
        branch: String,
    },
}

pub fn handle_tasks(
    task_rx: Receiver<Task>,
    transition_queue: &Arc<Mutex<VecDeque<Transition>>>,
) -> JoinHandle<()> {
    spawn({
        let transition_queue = Arc::clone(transition_queue);

        move || {
            while let Ok(task) = task_rx.recv() {
                let transition = match handle_task(task) {
                    Ok(transition) => transition,
                    Err(error) => Transition::Error(error.to_string()),
                };

                transition_queue
                    .lock()
                    .expect("failed to acquire transition queue")
                    .push_back(transition);
            }
        }
    })
}

fn handle_task(task: Task) -> Result<Transition> {
    match task {
        Task::GitSync {
            path,
            remote,
            branch,
        } => {
            let message = format!("[Task::GitSync] remote: {remote}, branch: {branch}");

            let mut storage = GitStorage::open(&path, StorageType::File)?;
            storage.set_remote(remote);
            storage.set_branch(branch);
            storage.pull()?;
            storage.push()?;

            Ok(Transition::Log(message))
        }
    }
}
