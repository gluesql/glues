use {
    super::sub_actions::update_statusbar,
    crate::{logger::log, traits::*, views::note_sheet::render_note_sheet, Node},
    cursive::Cursive,
    glues_core::Event,
};

pub fn initialize(siv: &mut Cursive) {
    log("[actions::initialize] hi");

    siv.glues().dispatch(Event::Initialize).log_unwrap();

    let note_sheet = render_note_sheet(siv);
    Node::body().find(siv).add_layer(note_sheet);

    update_statusbar(siv);
}
