use {
    crate::{traits::*, views::notebook::note_sheet::render_note_sheet, Node},
    cursive::Cursive,
};

pub fn open_notebook(siv: &mut Cursive) {
    log!("[transitions::open_notebook] hi");

    let note_sheet = render_note_sheet(siv);
    Node::body().find(siv).add_layer(note_sheet);
}
