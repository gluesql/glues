use {
    crate::{
        components::{alert::render_alert, confirm::render_confirm, prompt::render_prompt},
        traits::*,
        transitions::handle_event,
    },
    cursive::{
        Cursive,
        {view::View, views::ViewRef},
    },
    glues_core::{
        state::{GetInner, State},
        Event, Glues,
    },
};

pub trait CursiveExt {
    fn glues(&mut self) -> &mut Glues;

    fn state<T>(&mut self) -> &T
    where
        State: GetInner<T>;

    fn dispatch<T: Into<Event>>(&mut self, event: T);

    fn find<V: View>(&mut self, id: &str) -> ViewRef<V>;

    fn confirm<F>(&mut self, message: String, on_confirm: F)
    where
        F: Fn(&mut Cursive) + Send + Sync + 'static;

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + Send + Sync + 'static;

    fn prompt<F>(&mut self, message: &str, on_submit: F)
    where
        F: Fn(&mut Cursive, &str) + Clone + Send + Sync + 'static;

    fn focus_on_next_tick(&mut self, name: String);
}

impl CursiveExt for Cursive {
    fn glues(&mut self) -> &mut Glues {
        self.user_data::<Glues>()
            .log_expect("[CursiveExt::glues] Glues must exist")
    }

    fn state<T>(&mut self) -> &T
    where
        State: GetInner<T>,
    {
        self.glues().state.get_inner().log_unwrap()
    }

    fn dispatch<T: Into<Event>>(&mut self, event: T) {
        handle_event(self, event.into());
    }

    fn find<V: View>(&mut self, id: &str) -> ViewRef<V> {
        let msg = format!("[CursiveExt::find] {id} must exist");
        self.find_name(id).log_expect(&msg)
    }

    fn confirm<F>(&mut self, message: String, on_confirm: F)
    where
        F: Fn(&mut Cursive) + Send + Sync + 'static,
    {
        let dialog = render_confirm(&message, on_confirm);
        self.add_layer(dialog);
    }

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + Send + Sync + 'static,
    {
        let dialog = render_alert(&message, on_close);
        self.add_layer(dialog);
    }

    fn prompt<F>(&mut self, message: &str, on_submit: F)
    where
        F: Fn(&mut Cursive, &str) + Clone + Send + Sync + 'static,
    {
        let dialog = render_prompt(message, on_submit);
        self.add_layer(dialog);
    }

    fn focus_on_next_tick(&mut self, name: String) {
        self.cb_sink()
            .send(Box::new(move |siv| {
                siv.focus_name(&name).log_unwrap();
            }))
            .log_unwrap();
    }
}
