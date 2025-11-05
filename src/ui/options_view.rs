use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::test::TestMode;

/// Render the options view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(10),  // Mode selection
            Constraint::Min(5),      // Instructions
        ])
        .split(area);

    render_mode_selection(f, app, chunks[0]);
    render_instructions(f, chunks[1]);
}

/// Render mode selection
fn render_mode_selection(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Test Mode")
        .style(Style::default().fg(Color::Cyan));

    let is_time_mode = matches!(app.test_mode, TestMode::Time(_));

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            if is_time_mode {
                Span::styled("▶ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("  ")
            },
            Span::styled(
                "30 Seconds",
                if is_time_mode {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            if !is_time_mode {
                Span::styled("▶ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("  ")
            },
            Span::styled(
                "30 Words",
                if !is_time_mode {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press 't' or 'w' to switch modes",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Render instructions
fn render_instructions(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Instructions")
        .style(Style::default().fg(Color::White));

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Keyboard Shortcuts:",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::raw("  t - Switch to Time mode (30 seconds)")),
        Line::from(Span::raw("  w - Switch to Words mode (30 words)")),
        Line::from(""),
        Line::from(Span::raw("  1 - Go to Test tab")),
        Line::from(Span::raw("  2 - Go to Stats tab")),
        Line::from(Span::raw("  3 - Go to Options tab")),
        Line::from(""),
        Line::from(Span::raw("  Tab - Next tab")),
        Line::from(Span::raw("  Esc / q - Quit application")),
        Line::from(""),
        Line::from(Span::styled(
            "  Note: Changing mode will reset the current test.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}
