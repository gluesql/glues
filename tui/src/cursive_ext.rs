use {
    crate::{
        components::{alert::render_alert, confirm::render_confirm, prompt::render_prompt},
        traits::*,
    },
    cursive::{
        Cursive,
        {view::View, views::ViewRef},
    },
    glues_core::Glues,
};

pub trait CursiveExt {
    fn glues(&mut self) -> &mut Glues;

    fn find<V: View>(&mut self, id: &str) -> ViewRef<V>;

    fn confirm<F>(&mut self, message: String, on_confirm: F)
    where
        F: Fn(&mut Cursive) + 'static;

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + 'static;

    fn prompt<F>(&mut self, message: &str, on_submit: F)
    where
        F: Fn(&mut Cursive, &str) + Clone + 'static;
}

impl CursiveExt for Cursive {
    fn glues(&mut self) -> &mut Glues {
        self.user_data::<Glues>()
            .log_expect("[CursiveExt::glues] Glues must exist")
    }

    fn find<V: View>(&mut self, id: &str) -> ViewRef<V> {
        let msg = format!("[CursiveExt::find] {id} must exist");
        self.find_name(id).log_expect(&msg)
    }

    fn confirm<F>(&mut self, message: String, on_confirm: F)
    where
        F: Fn(&mut Cursive) + 'static,
    {
        let dialog = render_confirm(&message, on_confirm);
        self.add_layer(dialog);
    }

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + 'static,
    {
        let dialog = render_alert(&message, on_close);
        self.add_layer(dialog);
    }

    fn prompt<F>(&mut self, message: &str, on_submit: F)
    where
        F: Fn(&mut Cursive, &str) + Clone + 'static,
    {
        let dialog = render_prompt(message, on_submit);
        self.add_layer(dialog);
    }
}
