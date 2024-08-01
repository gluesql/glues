use {
    crate::{traits::*, views::notes::note_sheet::render_note_sheet, Node},
    cursive::Cursive,
};

pub fn initialize(siv: &mut Cursive) {
    log!("[transitions::initialize] hi");

    let note_sheet = render_note_sheet(siv);
    Node::body().find(siv).add_layer(note_sheet);
}
