use {
    crate::{Result, Transition},
    /*gluesql::gluesql_git_storage::{GitStorage, StorageType},*/
    std::{
        collections::VecDeque,
        /*path::PathBuf,*/
        sync::{Arc, Mutex, mpsc::Receiver},
        thread::{JoinHandle, spawn},
    },
};

#[derive(Clone, Debug)]
pub enum Task {
    // GitSync {
    //     path: PathBuf,
    //     remote: String,
    //     branch: String,
    // },
}

pub fn handle_tasks(
    task_rx: Receiver<Task>,
    _transition_queue: &Arc<Mutex<VecDeque<Transition>>>,
) -> JoinHandle<()> {
    spawn(move || {
        while task_rx.recv().is_ok() {
            // background tasks are disabled
        }
    })
}

#[allow(dead_code)]
fn handle_task(_task: Task) -> Result<Transition> {
    Ok(Transition::Log("background tasks disabled".to_owned()))
}
