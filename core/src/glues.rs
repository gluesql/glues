use {
    crate::{
        db::Db,
        state::{EntryState, State},
        task::{handle_tasks, Task},
        Event, Result, Transition,
    },
    std::{
        collections::VecDeque,
        sync::{
            mpsc::{channel, Sender},
            Arc, Mutex,
        },
        thread::JoinHandle,
    },
};

pub struct Glues {
    pub db: Option<Db>,
    pub state: State,

    pub task_tx: Sender<Task>,
    pub task_handle: JoinHandle<()>,
    pub transition_queue: Arc<Mutex<VecDeque<Transition>>>,
}

impl Glues {
    pub async fn new() -> Self {
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
