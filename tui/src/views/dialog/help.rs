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
    let [area] = Layout::vertical([Length(37)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title("Help")
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(30), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let message = vec![
        Line::from("Glues offers six storage options to suit your needs:"),
        Line::raw(""),
        Line::from("Instant".white().on_dark_gray()),
        Line::raw("Data is stored in memory and only persists while the app is running. This is useful for testing purposes."),
        Line::raw(""),
        Line::from("CSV".white().on_dark_gray()),
        Line::raw("Notes are saved in CSV format when you provide a path, Glues will load existing data if available or create a new file if none exists."),
        Line::raw(""),
        Line::from("JSON".white().on_dark_gray()),
        Line::raw("Notes are stored in JSON format, specifically using JSONL (JSON Lines). This functions similarly to CSV storage, allowing you to provide a path to load or create data."),
        Line::raw(""),
        Line::from("File".white().on_dark_gray()),
        Line::raw("Glues uses a custom storage format where each note and directory is stored as separate files. This is ideal for users who prefer a more granular file-based approach."),
        Line::raw(""),
        Line::from("Git".white().on_dark_gray()),
        Line::raw("Git storage requires three inputs: `path`, `remote`, and `branch`."),
        Line::raw("The `path` should point to an existing local Git repository, similar to the file storage path. For example, you can clone a GitHub repository and use that path."),
        Line::raw("The `remote` and `branch` specify the target remote repository and branch for synchronization."),
        Line::raw("When you modify notes or directories, Glues will automatically sync changes with the specified remote repository."),
        Line::raw(""),
        Line::from("MongoDB".white().on_dark_gray()),
        Line::raw("MongoDB storage allows you to store your notes in a MongoDB database, providing a scalable and centralized solution for managing your notes."),
        Line::raw("You need to provide the MongoDB connection string and the database name. Glues will handle storing and retrieving notes from the specified database."),
        Line::raw("This option is ideal for users who prefer a centralized storage solution or need robust, reliable data storage."),
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
