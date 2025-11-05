mod app;
mod ui;
mod test;
mod profile;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use app::App;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    app.init_test();

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print any errors that occurred
    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    while app.running {
        // Draw UI
        terminal.draw(|f| {
            ui(f, app);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only process KeyPress events, ignore KeyRelease
                if key.kind == KeyEventKind::Press {
                    // Global keybindings
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.quit();
                            continue;
                        }
                        KeyCode::Tab => {
                            app.next_tab();
                            continue;
                        }
                        KeyCode::BackTab => {
                            app.prev_tab();
                            continue;
                        }
                        KeyCode::Char('1') => {
                            app.current_tab = ui::Tab::Test;
                            continue;
                        }
                        KeyCode::Char('2') => {
                            app.current_tab = ui::Tab::Stats;
                            continue;
                        }
                        KeyCode::Char('3') => {
                            app.current_tab = ui::Tab::Options;
                            continue;
                        }
                        _ => {}
                    }

                    // Tab-specific keybindings
                    if app.current_tab == ui::Tab::Test {
                        let mut should_reset = false;

                        if let Some(engine) = &mut app.test_engine {
                            match key.code {
                                KeyCode::Char(ch) => {
                                    // Only handle Space specially, other chars are normal input
                                    if ch == ' ' {
                                        engine.next_word();
                                    } else {
                                        engine.type_char(ch);
                                    }
                                }
                                KeyCode::Backspace => {
                                    engine.backspace();
                                }
                                KeyCode::Enter => {
                                    // Reset test on Enter
                                    should_reset = true;
                                }
                                _ => {}
                            }

                            // Check if test should auto-finish
                            let was_in_progress = engine.state == crate::test::TestState::InProgress;
                            if engine.should_auto_finish() && was_in_progress {
                                engine.finish();
                            }
                        }

                        // Save result after test finishes (outside the borrow)
                        if let Some(engine) = &app.test_engine {
                            if engine.state == crate::test::TestState::Finished {
                                // Only save once per test completion
                                app.save_test_result();
                            }
                        }

                        // Reset outside of the borrow
                        if should_reset {
                            app.reset_test();
                        }
                    }

                    // Options tab keybindings
                    if app.current_tab == ui::Tab::Options {
                        match key.code {
                            KeyCode::Char('t') => {
                                app.set_time_mode();
                            }
                            KeyCode::Char('w') => {
                                app.set_words_mode();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Render the UI
fn ui(f: &mut ratatui::Frame, app: &App) {
    // Split screen into tab bar and content area
    let (tabs_area, content_area) = ui::split_screen(f.area());

    // Render tabs
    ui::render_tabs(f, tabs_area, app.current_tab);

    // Render content based on current tab
    match app.current_tab {
        ui::Tab::Test => crate::ui::test_view::render(f, app, content_area),
        ui::Tab::Stats => crate::ui::stats_view::render(f, app, content_area),
        ui::Tab::Options => crate::ui::options_view::render(f, app, content_area),
    }
}
