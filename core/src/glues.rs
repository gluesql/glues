use {
    crate::{
        Event, Result, Transition,
        backend::CoreBackend,
        state::{EntryState, State},
        task::{Task, handle_tasks},
    },
    std::{
        collections::VecDeque,
        sync::{
            Arc, Mutex,
            mpsc::{Sender, channel},
        },
        thread::JoinHandle,
    },
};

pub struct Glues {
    pub db: Option<Box<dyn CoreBackend + Send>>,
    pub state: State,

    pub task_tx: Sender<Task>,
    pub task_handle: JoinHandle<()>,
    pub transition_queue: Arc<Mutex<VecDeque<Transition>>>,
}

impl Default for Glues {
    fn default() -> Self {
        Self::new()
    }
}

impl Glues {
    pub fn new() -> Self {
        let transition_queue = Arc::new(Mutex::new(VecDeque::new()));
        let (task_tx, task_rx) = channel();
        let task_handle = handle_tasks(task_rx, &transition_queue);

        Self {
            db: None,
            state: EntryState.into(),
            task_tx,
            task_handle,
            transition_queue,
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<Transition> {
        State::consume(self, event).await
    }
}
