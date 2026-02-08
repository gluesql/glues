use {edtui::clipboard::ClipboardTrait, std::cell::RefCell, std::rc::Rc};

/// A shared clipboard handle that can be cloned and used both inside
/// edtui (via `set_clipboard`) and externally to get/set yank text.
#[derive(Default, Clone)]
pub struct ClipboardHandle(Rc<RefCell<String>>);

impl ClipboardHandle {
    pub fn get_text(&self) -> String {
        self.0.borrow().clone()
    }

    pub fn set_text(&self, text: String) {
        *self.0.borrow_mut() = text;
    }
}

impl ClipboardTrait for ClipboardHandle {
    fn set_text(&mut self, text: String) {
        *self.0.borrow_mut() = text;
    }

    fn get_text(&mut self) -> String {
        self.0.borrow().clone()
    }
}
