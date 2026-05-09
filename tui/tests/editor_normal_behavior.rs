#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

#[tokio::test]
async fn paste_after_multi_line_delete_restores_deleted_lines() -> Result<()> {
    let lines = sample_lines();
    let cases = [("dd", 1usize), ("2dd", 2), ("d2d", 2), ("3dd", 3)];

    for (command, deleted_count) in cases {
        let mut t = Tester::new().await?;
        t.open_instant().await?;
        t.open_first_note().await?;

        set_note_content(&mut t, &lines).await;

        for key in command.chars() {
            t.press(key).await;
        }
        t.press('p').await;

        let expected = expected_after_delete_then_paste(&lines, deleted_count);

        assert_eq!(
            t.editor_text(),
            expected,
            "delete command '{command}' did not preserve deleted lines in paste output"
        );
    }

    Ok(())
}

#[tokio::test]
async fn overflow_counted_dd_keeps_clipboard_and_paste_in_sync() -> Result<()> {
    let lines = sample_lines();

    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;
    set_note_content(&mut t, &lines).await;

    for _ in 0..3 {
        t.press('j').await;
    }

    t.press('5').await;
    t.press('d').await;
    t.press('d').await;

    assert_eq!(
        t.clipboard_text(),
        "\nline 4 delta\nline 5 epsilon\nline 3 gamma\nline 2 beta\nline 1 alpha"
    );

    t.press('p').await;

    assert_eq!(
        t.editor_text(),
        "line 4 delta\nline 5 epsilon\nline 3 gamma\nline 2 beta\nline 1 alpha"
    );

    Ok(())
}

#[tokio::test]
async fn overflow_d_count_k_keeps_clipboard_and_paste_in_sync() -> Result<()> {
    let lines = sample_lines();

    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;
    set_note_content(&mut t, &lines).await;

    t.press('j').await;

    t.press('d').await;
    t.press('4').await;
    t.press('k').await;

    assert_eq!(
        t.clipboard_text(),
        "\nline 1 alpha\nline 2 beta\nline 3 gamma\nline 4 delta\nline 5 epsilon"
    );

    t.press('p').await;

    assert_eq!(
        t.editor_text(),
        "line 1 alpha\nline 2 beta\nline 3 gamma\nline 4 delta\nline 5 epsilon"
    );

    Ok(())
}

fn sample_lines() -> [&'static str; 5] {
    [
        "line 1 alpha",
        "line 2 beta",
        "line 3 gamma",
        "line 4 delta",
        "line 5 epsilon",
    ]
}

async fn set_note_content(tester: &mut Tester, lines: &[&str]) {
    tester.press('d').await;
    tester.press('d').await;
    tester.press('i').await;

    for (idx, line) in lines.iter().enumerate() {
        tester.type_str(line).await;

        if idx + 1 < lines.len() {
            tester.key(KeyCode::Enter).await;
        }
    }

    tester.key(KeyCode::Esc).await;
    tester.press('g').await;
    tester.press('g').await;
}

fn expected_after_delete_then_paste(lines: &[&str], deleted_count: usize) -> String {
    let deleted_count = deleted_count.min(lines.len());

    if deleted_count == 0 || deleted_count >= lines.len() {
        return lines.join("\n");
    }

    let mut out = Vec::with_capacity(lines.len());
    out.push(lines[deleted_count]);
    out.extend_from_slice(&lines[..deleted_count]);
    out.extend_from_slice(&lines[deleted_count + 1..]);
    out.join("\n")
}
