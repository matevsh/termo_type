use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::App;
use crate::test::{CharState, TestState};

/// Render the test view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let Some(engine) = &app.test_engine else {
        render_no_test(f, area);
        return;
    };

    // Split into stats area and content area
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),  // Stats bar
            Constraint::Min(10),    // Words display (3 lines centered)
            Constraint::Length(3),  // Help text
        ])
        .split(area);

    // Render stats bar
    render_stats_bar(f, app, engine, chunks[0]);

    // Render 3-line words display
    render_words_three_lines(f, engine, chunks[1]);

    // Render help/instructions
    render_help(f, engine, chunks[2]);
}

/// Render when test engine is not initialized
fn render_no_test(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Test")
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new("Loading test...")
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}

/// Render the stats bar with metrics
fn render_stats_bar(f: &mut Frame, _app: &App, engine: &crate::test::TestEngine, area: Rect) {
    let metrics = engine.get_metrics();

    // Calculate time remaining or elapsed
    let time_display = match engine.mode {
        crate::test::TestMode::Time(seconds) => {
            let remaining = (seconds as f64 - engine.elapsed_seconds()).max(0.0);
            format!("Time: {:.1}s", remaining)
        }
        crate::test::TestMode::Words(_) => {
            format!("Time: {:.1}s", engine.elapsed_seconds())
        }
    };

    // Calculate progress
    let progress_display = match engine.mode {
        crate::test::TestMode::Time(_) => {
            format!("Words: {}", engine.current_word_index)
        }
        crate::test::TestMode::Words(count) => {
            format!("Progress: {}/{}", engine.current_word_index, count)
        }
    };

    let stats_text = format!(
        " {} | {} | WPM: {:.0} | CPM: {:.0} | Accuracy: {:.1}% ",
        time_display,
        progress_display,
        metrics.wpm,
        metrics.cpm,
        metrics.accuracy
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(stats_text)
        .block(block)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

/// Render 3 lines of words centered on screen
fn render_words_three_lines(f: &mut Frame, engine: &crate::test::TestEngine, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Type the words")
        .style(Style::default().fg(Color::White));

    // Get current line and the next 2 lines
    let current_line_idx = engine.current_line_index;
    let lines_to_display = 3;

    // Calculate cursor blink state (530ms on, 530ms off) for smooth blinking
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let show_cursor = (now % 1060) < 530;

    let mut display_lines: Vec<Line> = Vec::new();

    // Add padding lines to center the content
    let total_height = area.height.saturating_sub(2); // Subtract borders
    let content_lines = lines_to_display;
    let padding_top = (total_height.saturating_sub(content_lines as u16)) / 2;

    for _ in 0..padding_top {
        display_lines.push(Line::from(""));
    }

    // Render 3 lines
    for line_offset in 0..lines_to_display {
        let line_idx = current_line_idx + line_offset;

        if line_idx >= engine.lines.len() {
            display_lines.push(Line::from(""));
            continue;
        }

        let line_words = &engine.lines[line_idx];
        let mut line_spans: Vec<Span> = Vec::new();

        // Determine if this is the current line
        let is_current_line = line_offset == 0;

        for (word_idx_in_line, word) in line_words.iter().enumerate() {
            // Add space before word (except first)
            if !line_spans.is_empty() {
                line_spans.push(Span::raw(" "));
            }

            // Calculate global word index
            let global_word_idx = line_idx * engine.words_per_line + word_idx_in_line;

            // Check if this is the current word being typed
            let is_current_word = is_current_line && word_idx_in_line == engine.current_word_in_line;

            // Check if word was already typed
            let is_typed = global_word_idx < engine.current_word_index;

            if is_current_word {
                // Current word being typed - render with live feedback
                if let Some(word_state) = &engine.current_word_state {
                    for (char_idx, ch) in word.chars().enumerate() {
                        // Add cursor BEFORE the current character
                        if show_cursor && char_idx == word_state.cursor_pos {
                            line_spans.push(Span::styled("|", Style::default().fg(Color::Yellow)));
                        }

                        let style = if char_idx < word_state.char_states.len() {
                            match word_state.char_states[char_idx] {
                                CharState::Correct => Style::default().fg(Color::White),
                                CharState::Incorrect => Style::default().fg(Color::LightRed),
                                CharState::Untyped => Style::default().fg(Color::Gray),
                            }
                        } else {
                            Style::default().fg(Color::Gray)
                        };

                        line_spans.push(Span::styled(ch.to_string(), style));
                    }

                    // If cursor is at the end of the word, add it after
                    if show_cursor && word_state.cursor_pos >= word.chars().count() {
                        line_spans.push(Span::styled("|", Style::default().fg(Color::Yellow)));
                    }
                }
            } else if is_typed {
                // Already typed word - show in white (or red if had errors)
                let had_errors = engine.word_had_errors.get(global_word_idx).copied().unwrap_or(false);
                let color = if had_errors {
                    Color::LightRed
                } else {
                    Color::White
                };
                line_spans.push(Span::styled(word.clone(), Style::default().fg(color)));
            } else {
                // Future word - show in gray (darker for line 2)
                let color = if line_offset == 1 {
                    Color::DarkGray
                } else if line_offset == 2 {
                    Color::Rgb(60, 60, 60) // Even darker gray
                } else {
                    Color::Gray
                };
                line_spans.push(Span::styled(word.clone(), Style::default().fg(color)));
            }
        }

        display_lines.push(Line::from(line_spans));
    }

    let paragraph = Paragraph::new(display_lines)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

/// Render help text
fn render_help(f: &mut Frame, engine: &crate::test::TestEngine, area: Rect) {
    let help_text = match engine.state {
        TestState::NotStarted => "Start typing to begin | Enter: Reset | Tab: Change tab | Esc: Quit",
        TestState::InProgress => "Type the words | Space: Next word | Enter: Reset | Tab: Change tab | Esc: Quit",
        TestState::Finished => "Test finished! | Enter: Reset | Tab: View stats | Esc: Quit",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(paragraph, area);
}
