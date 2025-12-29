//! # TUI Dashboard Command
//!
//! Interactive terminal dashboard using ratatui.
//!
//! ## Example
//! ```bash
//! dx ui                # Launch dashboard
//! dx ui --tick 500     # Slower refresh rate
//! ```

use crate::cli::commands::ui::UiArgs;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use sysinfo::System;

pub fn run(args: UiArgs) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let tick_rate = Duration::from_millis(args.tick);

    // Run main loop
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

struct App {
    sys: System,
    should_quit: bool,
    last_refresh: Instant,
}

impl App {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys,
            should_quit: false,
            last_refresh: Instant::now(),
        }
    }

    fn refresh(&mut self) {
        if self.last_refresh.elapsed() >= Duration::from_secs(1) {
            self.sys.refresh_all();
            self.last_refresh = Instant::now();
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('r') => {
                            app.sys.refresh_all();
                            app.last_refresh = Instant::now();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.refresh();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Header
    render_header(f, chunks[0]);

    // Main content
    render_main(f, chunks[1], app);

    // Footer
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let now = chrono::Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let title = Line::from(vec![
        Span::styled(
            " dx ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Developer Experience CLI "),
        Span::styled(
            format!(" v{} ", env!("CARGO_PKG_VERSION")),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw(" | "),
        Span::styled(time_str, Style::default().fg(Color::Yellow)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(title).block(block);
    f.render_widget(paragraph, area);
}

fn render_main(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left panel: System info
    render_system_info(f, chunks[0], app);

    // Right panel: Quick stats
    render_quick_stats(f, chunks[1], app);
}

fn render_system_info(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" System Info ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // CPU
            Constraint::Length(2), // Memory
            Constraint::Length(2), // Uptime
            Constraint::Min(0),    // Remaining
        ])
        .split(inner);

    // CPU usage
    let cpu_usage = app.sys.global_cpu_usage();
    let cpu_gauge = Gauge::default()
        .label(format!("CPU: {:.1}%", cpu_usage))
        .ratio((cpu_usage / 100.0) as f64)
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    f.render_widget(cpu_gauge, chunks[0]);

    // Memory usage
    let total_mem = app.sys.total_memory();
    let used_mem = app.sys.used_memory();
    let mem_percent = if total_mem > 0 {
        (used_mem as f64 / total_mem as f64) * 100.0
    } else {
        0.0
    };
    let mem_gauge = Gauge::default()
        .label(format!(
            "Memory: {:.1}% ({} / {})",
            mem_percent,
            format_bytes(used_mem),
            format_bytes(total_mem)
        ))
        .ratio(mem_percent / 100.0)
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray));
    f.render_widget(mem_gauge, chunks[1]);

    // Uptime
    let uptime = System::uptime();
    let uptime_str = format_duration(uptime);
    let uptime_text = Paragraph::new(Line::from(vec![
        Span::styled("Uptime: ", Style::default().fg(Color::DarkGray)),
        Span::styled(uptime_str, Style::default().fg(Color::Yellow)),
    ]));
    f.render_widget(uptime_text, chunks[2]);
}

fn render_quick_stats(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Quick Stats ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_default();
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let cpu_count = app.sys.cpus().len();

    let text = vec![
        Line::from(vec![
            Span::styled("Host:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(hostname, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("OS:       ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} {}", os_name, os_version),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("CPUs:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(cpu_count.to_string(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Processes: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.sys.processes().len().to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::NONE));
    f.render_widget(
        paragraph,
        Rect::new(inner.x + 1, inner.y + 1, inner.width - 2, inner.height - 2),
    );
}

fn render_footer(f: &mut Frame, area: Rect) {
    let keybindings = Line::from(vec![
        Span::styled(" q ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Quit  "),
        Span::styled(" r ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Refresh  "),
        Span::styled(" Esc ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Exit"),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(keybindings).block(block);
    f.render_widget(paragraph, area);
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}
