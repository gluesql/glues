use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn close_directory(siv: &mut Cursive, directory_id: DirectoryId, by_note: bool) {
    let directory_node = Node::notebook().note_tree().directory(&directory_id);

    directory_node.caret().find(siv).set_content("▸ ");
    let mut container = directory_node.find(siv);
    let n = container.len();
    for _ in 1..n {
        container.remove_child(1);
    }

    if by_note {
        siv.focus_on_next_tick(
            Node::notebook()
                .note_tree()
                .directory(&directory_id)
                .name_button()
                .name(),
        );
    }
}
