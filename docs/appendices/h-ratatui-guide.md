# Appendix H: Terminal UI with Ratatui

[Ratatui](https://ratatui.rs/) is a Rust library for building terminal user interfaces.

## Overview

Ratatui provides:
- Immediate mode rendering
- Layout system (constraints, splits)
- Built-in widgets (paragraphs, gauges, lists, tables)
- Styling (colors, modifiers)
- Cross-platform support via backends (crossterm, termion)

## Basic Setup

### Dependencies

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
```

### Minimal Example

```rust
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode,
               EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let block = Block::default()
                .title("Hello Ratatui")
                .borders(Borders::ALL);
            f.render_widget(block, f.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
```

## Layout System

Ratatui uses constraints to define layouts:

```rust
use ratatui::layout::{Constraint, Direction, Layout};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),      // Fixed 3 rows
        Constraint::Min(10),        // At least 10, expands
        Constraint::Percentage(20), // 20% of space
    ])
    .split(f.area());

// chunks[0] = header area
// chunks[1] = main content
// chunks[2] = footer
```

### Horizontal Split

```rust
let columns = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(area);
```

## Common Widgets

### Paragraph

```rust
use ratatui::widgets::Paragraph;
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

let text = vec![
    Line::from(vec![
        Span::styled("Hello ", Style::default().fg(Color::Green)),
        Span::raw("World"),
    ]),
];

let paragraph = Paragraph::new(text)
    .block(Block::default().title("Greeting").borders(Borders::ALL));

f.render_widget(paragraph, area);
```

### Gauge (Progress Bar)

```rust
use ratatui::widgets::Gauge;

let gauge = Gauge::default()
    .block(Block::default().title("CPU"))
    .gauge_style(Style::default().fg(Color::Cyan))
    .percent(45)
    .label("45%");

f.render_widget(gauge, area);
```

### List

```rust
use ratatui::widgets::{List, ListItem};

let items: Vec<ListItem> = vec!["Item 1", "Item 2", "Item 3"]
    .into_iter()
    .map(ListItem::new)
    .collect();

let list = List::new(items)
    .block(Block::default().title("Items").borders(Borders::ALL))
    .highlight_style(Style::default().bg(Color::Blue));

f.render_widget(list, area);
```

### Table

```rust
use ratatui::widgets::{Table, Row, Cell};

let rows = vec![
    Row::new(vec!["Name", "Value"]),
    Row::new(vec!["CPU", "45%"]),
    Row::new(vec!["Memory", "62%"]),
];

let table = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
    .block(Block::default().title("Stats").borders(Borders::ALL));

f.render_widget(table, area);
```

## Styling

```rust
use ratatui::style::{Color, Modifier, Style};

// Colors
Style::default().fg(Color::Red)
Style::default().bg(Color::Blue)
Style::default().fg(Color::Rgb(255, 128, 0))

// Modifiers
Style::default().add_modifier(Modifier::BOLD)
Style::default().add_modifier(Modifier::ITALIC)
Style::default().add_modifier(Modifier::UNDERLINED)

// Combined
Style::default()
    .fg(Color::Yellow)
    .bg(Color::Black)
    .add_modifier(Modifier::BOLD)
```

## Event Handling

```rust
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

// Poll with timeout
if event::poll(Duration::from_millis(100))? {
    if let Event::Key(key) = event::read()? {
        // Only handle key press, not release
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => refresh(),
                KeyCode::Up => scroll_up(),
                KeyCode::Down => scroll_down(),
                _ => {}
            }
        }
    }
}
```

## The dx ui Implementation

The `dx ui` command uses this pattern:

```rust
struct App {
    sys: System,
    should_quit: bool,
}

fn run_app(terminal: &mut Terminal<impl Backend>, app: &mut App) {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('r') => app.sys.refresh_all(),
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }

        app.sys.refresh_all();
    }
}

fn ui(f: &mut Frame, app: &App) {
    // Header, main content, footer layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_main(f, chunks[1], app);
    render_footer(f, chunks[2]);
}
```

## Resources

- [Ratatui Book](https://ratatui.rs/tutorials/)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [Widget Gallery](https://ratatui.rs/showcase/widgets/)
