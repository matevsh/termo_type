use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Render the stats view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(12),  // Best 30 seconds
            Constraint::Length(12),  // Best 30 words
            Constraint::Min(3),      // Info
        ])
        .split(area);

    render_best_score(f, "Best 30 Seconds", &app.profile.best_30_seconds, chunks[0]);
    render_best_score(f, "Best 30 Words", &app.profile.best_30_words, chunks[1]);
    render_info(f, chunks[2]);
}

/// Render a best score card
fn render_best_score(
    f: &mut Frame,
    title: &str,
    score: &Option<crate::profile::BestScore>,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::Cyan));

    let content = if let Some(score) = score {
        // Format timestamp
        let timestamp = chrono::DateTime::from_timestamp(score.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  WPM: "),
                Span::styled(
                    format!("{:.0}", score.wpm),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  CPM: "),
                Span::styled(
                    format!("{:.0}", score.cpm),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Accuracy: "),
                Span::styled(
                    format!("{:.1}%", score.accuracy),
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Date: "),
                Span::styled(
                    timestamp,
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "  No score yet!",
                Style::default().fg(Color::Gray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Complete a test to set your",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  first record.",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    };

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Render info section
fn render_info(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Info")
        .style(Style::default().fg(Color::White));

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Your best scores are automatically saved!",
            Style::default().fg(Color::Green),
        )),
        Line::from(""),
        Line::from(Span::raw(
            "  Complete tests in 30-second or 30-word modes",
        )),
        Line::from(Span::raw(
            "  to compete with your personal bests.",
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Profile location:",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            format!("  {}", crate::profile::storage::get_profile_path_display()),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}
