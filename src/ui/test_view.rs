use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
            Constraint::Min(5),     // Words display
            Constraint::Length(3),  // Help text
        ])
        .split(area);

    // Render stats bar
    render_stats_bar(f, app, engine, chunks[0]);

    // Render words
    render_words(f, engine, chunks[1]);

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
fn render_stats_bar(f: &mut Frame, app: &App, engine: &crate::test::TestEngine, area: Rect) {
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

/// Render the words to type with coloring
fn render_words(f: &mut Frame, engine: &crate::test::TestEngine, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // Show current and next few words
    let words_to_show = 20;
    let start_index = engine.current_word_index;
    let end_index = (start_index + words_to_show).min(engine.words.len());

    // Build spans for all visible words
    let mut current_line_spans: Vec<Span> = Vec::new();
    let mut current_line_length = 0;
    const MAX_LINE_LENGTH: usize = 60;

    for word_index in start_index..end_index {
        let word = &engine.words[word_index];

        // Add space before word (except first word in line)
        if !current_line_spans.is_empty() {
            current_line_spans.push(Span::raw(" "));
            current_line_length += 1;
        }

        // Check if we need to wrap to next line
        if current_line_length + word.len() > MAX_LINE_LENGTH && !current_line_spans.is_empty() {
            lines.push(Line::from(current_line_spans.clone()));
            current_line_spans.clear();
            current_line_length = 0;
        }

        // Render current word being typed
        if word_index == engine.current_word_index {
            if let Some(word_state) = &engine.current_word_state {
                for (char_index, ch) in word.chars().enumerate() {
                    let style = if char_index < word_state.char_states.len() {
                        match word_state.char_states[char_index] {
                            CharState::Correct => Style::default().fg(Color::Green),
                            CharState::Incorrect => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            CharState::Untyped => Style::default().fg(Color::Gray),
                        }
                    } else {
                        Style::default().fg(Color::Gray)
                    };

                    // Add cursor on current character
                    let style = if char_index == word_state.cursor_pos {
                        style.bg(Color::White).fg(Color::Black)
                    } else {
                        style
                    };

                    current_line_spans.push(Span::styled(ch.to_string(), style));
                }

                // Show cursor at end if word is complete
                if word_state.cursor_pos >= word.len() {
                    current_line_spans.push(Span::styled(" ", Style::default().bg(Color::White)));
                }

                current_line_length += word.len();
            }
        } else if word_index < engine.current_word_index {
            // Already typed words - show in dimmed color
            current_line_spans.push(Span::styled(
                word.clone(),
                Style::default().fg(Color::DarkGray),
            ));
            current_line_length += word.len();
        } else {
            // Future words - show in default color
            current_line_spans.push(Span::styled(
                word.clone(),
                Style::default().fg(Color::White),
            ));
            current_line_length += word.len();
        }
    }

    // Add remaining spans to last line
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Type the words")
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);

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
