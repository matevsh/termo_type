use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Tabs},
    Frame,
};

/// Available tabs in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Test,
    Stats,
    Options,
}

impl Tab {
    /// Get all available tabs
    pub fn all() -> Vec<Tab> {
        vec![Tab::Test, Tab::Stats, Tab::Options]
    }

    /// Get the display name of the tab
    pub fn name(&self) -> &str {
        match self {
            Tab::Test => "Test",
            Tab::Stats => "Statystyki",
            Tab::Options => "Opcje",
        }
    }

    /// Get the next tab (cycling)
    pub fn next(&self) -> Tab {
        match self {
            Tab::Test => Tab::Stats,
            Tab::Stats => Tab::Options,
            Tab::Options => Tab::Test,
        }
    }

    /// Get the previous tab (cycling)
    pub fn prev(&self) -> Tab {
        match self {
            Tab::Test => Tab::Options,
            Tab::Stats => Tab::Test,
            Tab::Options => Tab::Stats,
        }
    }
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Test
    }
}

/// Render the tab bar at the top of the screen
pub fn render_tabs(f: &mut Frame, area: Rect, current_tab: Tab) {
    let tab_list = Tab::all();
    let titles: Vec<Span> = tab_list
        .iter()
        .map(|t| Span::raw(t.name()))
        .collect();

    let current_index = match current_tab {
        Tab::Test => 0,
        Tab::Stats => 1,
        Tab::Options => 2,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title("TermoType - Typing Speed Test")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .select(current_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

/// Split the screen into tab bar and content area
pub fn split_screen(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    (chunks[0], chunks[1])
}
