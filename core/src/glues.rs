#[cfg(not(target_arch = "wasm32"))]
mod native {
    use {
        crate::{
            Event, Result, Transition,
            backend::BackendBox,
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
        pub db: Option<BackendBox>,
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

    pub use Glues as Inner;
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{
        Event, Result, Transition,
        backend::BackendBox,
        state::{EntryState, State},
    };

    pub struct Glues {
        pub db: Option<BackendBox>,
        pub state: State,
    }

    impl Default for Glues {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Glues {
        pub fn new() -> Self {
            Self {
                db: None,
                state: EntryState.into(),
            }
        }

        pub async fn dispatch(&mut self, event: Event) -> Result<Transition> {
            State::consume(self, event).await
        }
    }

    pub use Glues as Inner;
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::Inner as Glues;
#[cfg(target_arch = "wasm32")]
pub use wasm::Inner as Glues;
