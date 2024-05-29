use {
    crate::{
        components::{alert::render_alert, confirm::render_confirm},
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
        F: Fn(&mut Cursive) + Send + 'static;

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + Send + 'static;
}

impl CursiveExt for Cursive {
    fn glues(&mut self) -> &mut Glues {
        self.user_data::<Glues>().log_expect("Glues must exist")
    }

    fn find<V: View>(&mut self, id: &str) -> ViewRef<V> {
        self.find_name(id).log_expect("View with {id} must exist")
    }

    fn confirm<F>(&mut self, message: String, on_confirm: F)
    where
        F: Fn(&mut Cursive) + Send + 'static,
    {
        let dialog = render_confirm(&message, on_confirm);
        self.add_layer(dialog);
    }

    fn alert<F>(&mut self, message: String, on_close: F)
    where
        F: Fn(&mut Cursive) + Send + 'static,
    {
        let dialog = render_alert(&message, on_close);
        self.add_layer(dialog);
    }
}
