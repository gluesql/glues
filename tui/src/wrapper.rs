use cursive::{
    event::{Event, EventResult, Key},
    view::ViewWrapper,
    View,
};

pub struct HjklWrapper<T> {
    view: T,
}

impl<T> HjklWrapper<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    cursive::inner_getters!(self.view: T);
}

impl<T> ViewWrapper for HjklWrapper<T>
where
    T: View,
{
    cursive::wrap_impl!(self.view: T);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        let event = match event {
            Event::Char('h') => Key::Left.into(),
            Event::Char('j') => Key::Down.into(),
            Event::Char('k') => Key::Up.into(),
            Event::Char('l') => Key::Right.into(),
            _ => event,
        };

        self.view.on_event(event.clone())
    }
}
