//! DHIS2 TUI browser for organisation units.
//!
//! Controls:
//! - j/k or ↓/↑: Navigate list
//! - Enter/l: Drill down into children
//! - Backspace/h: Go to parent
//! - g/G: Go to first/last item
//! - /: Search (type to filter)
//! - q: Quit

#![cfg(feature = "ui")]

use super::client::Dhis2Client;
use super::org_units::OrgUnit;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use serde::Deserialize;
use std::collections::HashMap;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Deserialize)]
struct OrgUnitApiResponse {
    #[serde(rename = "organisationUnits")]
    organisation_units: Vec<OrgUnit>,
}

#[derive(PartialEq)]
enum Mode {
    Browse,
    Search,
}

struct App {
    client: Dhis2Client,
    org_units: Vec<OrgUnit>,
    filtered_units: Vec<OrgUnit>,
    cache: HashMap<Option<String>, Vec<OrgUnit>>,
    state: ListState,
    scroll_state: ScrollbarState,
    current_parent_id: Option<String>,
    breadcrumb: Vec<(String, String)>,
    error: Option<String>,
    status: String,
    mode: Mode,
    search_query: String,
}

impl App {
    fn new(client: Dhis2Client) -> Result<Self> {
        let mut app = Self {
            client,
            org_units: Vec::new(),
            filtered_units: Vec::new(),
            cache: HashMap::new(),
            state: ListState::default(),
            scroll_state: ScrollbarState::default(),
            current_parent_id: None,
            breadcrumb: Vec::new(),
            error: None,
            status: "Loading...".to_string(),
            mode: Mode::Browse,
            search_query: String::new(),
        };
        app.load_children(None)?;
        Ok(app)
    }

    fn load_children(&mut self, parent_id: Option<&str>) -> Result<()> {
        self.error = None;
        self.status = "Loading...".to_string();

        let cache_key = parent_id.map(|s| s.to_string());

        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key) {
            self.org_units = cached.clone();
            self.apply_filter();
            self.update_status();
            return Ok(());
        }

        // Fetch from API
        let fields = "id,displayName,name,level,path,parent[id,displayName],featureType";
        let url = match parent_id {
            Some(pid) => format!(
                "organisationUnits.json?fields={}&filter=parent.id:eq:{}&paging=false",
                fields, pid
            ),
            None => format!(
                "organisationUnits.json?fields={}&filter=level:eq:1&paging=false",
                fields
            ),
        };

        match self.client.get::<OrgUnitApiResponse>(&url) {
            Ok(response) => {
                self.org_units = response.organisation_units;
                // Sort by name
                self.org_units.sort_by(|a, b| a.display_name.cmp(&b.display_name));
                // Cache the result
                self.cache.insert(cache_key, self.org_units.clone());
                self.apply_filter();
                self.update_status();
            }
            Err(e) => {
                self.error = Some(format!("Failed to load: {}", e));
            }
        }

        Ok(())
    }

    fn apply_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_units = self.org_units.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_units = self
                .org_units
                .iter()
                .filter(|ou| ou.display_name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
        self.scroll_state = self.scroll_state.content_length(self.filtered_units.len());
        if !self.filtered_units.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
    }

    fn update_status(&mut self) {
        let count = self.filtered_units.len();
        let total = self.org_units.len();
        if self.search_query.is_empty() {
            self.status = format!("{} items", count);
        } else {
            self.status = format!("{}/{} matching \"{}\"", count, total, self.search_query);
        }
    }

    fn selected_org_unit(&self) -> Option<&OrgUnit> {
        self.state
            .selected()
            .and_then(|i| self.filtered_units.get(i))
    }

    fn drill_down(&mut self) -> Result<()> {
        if let Some(ou) = self.selected_org_unit().cloned() {
            self.breadcrumb.push((ou.id.clone(), ou.display_name.clone()));
            self.current_parent_id = Some(ou.id.clone());
            self.search_query.clear();
            self.load_children(Some(&ou.id))?;
        }
        Ok(())
    }

    fn go_up(&mut self) -> Result<()> {
        if let Some((_, _)) = self.breadcrumb.pop() {
            self.current_parent_id = self.breadcrumb.last().map(|(id, _)| id.clone());
            self.search_query.clear();
            self.load_children(self.current_parent_id.as_deref())?;
        }
        Ok(())
    }

    fn next(&mut self) {
        let len = self.filtered_units.len();
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1).min(len - 1),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn previous(&mut self) {
        let len = self.filtered_units.len();
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn first(&mut self) {
        if !self.filtered_units.is_empty() {
            self.state.select(Some(0));
            self.scroll_state = self.scroll_state.position(0);
        }
    }

    fn last(&mut self) {
        let len = self.filtered_units.len();
        if len > 0 {
            self.state.select(Some(len - 1));
            self.scroll_state = self.scroll_state.position(len - 1);
        }
    }
}

// ============================================================================
// UI Rendering
// ============================================================================

fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/breadcrumb
            Constraint::Min(5),    // Main list
            Constraint::Length(3), // Details
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    // Header with breadcrumb
    render_header(frame, app, chunks[0]);

    // Main list
    render_list(frame, app, chunks[1]);

    // Details panel
    render_details(frame, app, chunks[2]);

    // Status bar
    render_status(frame, app, chunks[3]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![Span::styled("Root", Style::default().fg(Color::Cyan))];

    for (_, name) in &app.breadcrumb {
        spans.push(Span::raw(" > "));
        spans.push(Span::styled(name.as_str(), Style::default().fg(Color::Cyan)));
    }

    let title = if app.mode == Mode::Search {
        format!(" DHIS2 Browser [/{}] ", app.search_query)
    } else {
        " DHIS2 Organisation Units ".to_string()
    };

    let header = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(header, area);
}

fn render_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_units
        .iter()
        .map(|ou| {
            let level_color = match ou.level {
                1 => Color::Green,
                2 => Color::Yellow,
                3 => Color::Cyan,
                4 => Color::Magenta,
                _ => Color::White,
            };

            let geom_indicator = match ou.feature_type.as_deref() {
                Some("POINT") => " ●",
                Some("POLYGON") | Some("MULTI_POLYGON") => " ◻",
                _ => "",
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("L{} ", ou.level), Style::default().fg(level_color)),
                Span::raw(&ou.display_name),
                Span::styled(geom_indicator, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} items ",
            app.filtered_units.len()
        )))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    frame.render_stateful_widget(list, area, &mut app.state);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    frame.render_stateful_widget(
        scrollbar,
        area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scroll_state,
    );
}

fn render_details(frame: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(ou) = app.selected_org_unit() {
        let parent = ou
            .parent
            .as_ref()
            .and_then(|p| p.display_name.as_deref())
            .unwrap_or("-");
        format!(
            "ID: {}  |  Level: {}  |  Parent: {}  |  Path: {}",
            ou.id, ou.level, parent, ou.path
        )
    } else if let Some(err) = &app.error {
        format!("Error: {}", err)
    } else {
        "No selection".to_string()
    };

    let details = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Details "));
    frame.render_widget(details, area);
}

fn render_status(frame: &mut Frame, app: &App, area: Rect) {
    let status = Paragraph::new(format!(
        " {} │ ↑↓/jk:Navigate Enter/l:Open h/←:Back /:Search g/G:First/Last q:Quit",
        app.status
    ))
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(status, area);
}

// ============================================================================
// Main Entry Point
// ============================================================================

/// Run the TUI browser.
pub fn run(client: Dhis2Client) -> Result<()> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create app
    let mut app = App::new(client)?;

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.mode == Mode::Search {
                match key.code {
                    KeyCode::Esc => {
                        app.mode = Mode::Browse;
                        app.search_query.clear();
                        app.apply_filter();
                    }
                    KeyCode::Enter => {
                        app.mode = Mode::Browse;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.apply_filter();
                    }
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                        app.apply_filter();
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('/') => {
                    app.mode = Mode::Search;
                }
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                    app.drill_down()?;
                }
                KeyCode::Backspace | KeyCode::Char('h') | KeyCode::Left => {
                    app.go_up()?;
                }
                KeyCode::Char('g') => app.first(),
                KeyCode::Char('G') => app.last(),
                _ => {}
            }
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
