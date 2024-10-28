use ratatui::{
    layout::{Alignment, Constraint::Length, Flex, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Clear, Padding, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame) {
    let [area] = Layout::horizontal([Length(120)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(34)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title("Help")
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(27), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let message = vec![
        Line::from("Glues offers various storage options to suit your needs:"),
        Line::raw(""),
        Line::from("Instant".white().on_dark_gray()),
        Line::raw("Data is stored in memory and only persists while the app is running."),
        Line::raw("This option is useful for testing or temporary notes as it is entirely volatile."),
        Line::raw(""),
        Line::from("Local".white().on_dark_gray()),
        Line::raw("Notes are stored locally as separate files."),
        Line::raw("This is the default option for users who prefer a simple, file-based approach without any remote synchronization."),
        Line::raw(""),
        Line::from("Git".white().on_dark_gray()),
        Line::raw("Git storage requires three inputs: `path`, `remote`, and `branch`."),
        Line::raw("The `path` should point to an existing local Git repository, similar to the file storage path."),
        Line::raw("For example, you can clone a GitHub repository and use that path."),
        Line::raw("The `remote` and `branch` specify the target remote repository and branch for synchronization."),
        Line::raw("When you modify notes or directories, Glues will automatically sync changes with the specified remote repository."),
        Line::raw(""),
        Line::from("MongoDB".white().on_dark_gray()),
        Line::raw("MongoDB storage allows you to store your notes in a MongoDB database, providing a scalable and centralized solution for managing your notes."),
        Line::raw("You need to provide the MongoDB connection string and the database name."),
        Line::raw("Glues will handle storing and retrieving notes from the specified database."),
        Line::raw("This option is ideal for users who prefer a centralized storage solution or need robust, reliable data storage."),
        Line::raw(""),
        Line::from(vec![
            "CSV".white().on_dark_gray(),
            " or ".dim(),
            "JSON".white().on_dark_gray(),
        ]),
        Line::raw("These formats store notes as simple log files, ideal for quick data exports or reading logs."),
        Line::raw("CSV saves data in comma-separated format, while JSON uses JSONL (JSON Lines) format."),
        Line::raw(""),
    ];
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);
    let control = Line::from("Press any key to close".dark_gray()).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
