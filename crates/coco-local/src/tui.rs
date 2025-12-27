//! Terminal dashboard for local mode.

use std::io::{self, Stdout};
use std::sync::Arc;
use std::time::Duration;

use coco_protocol::{CocoError, CocoResult};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};
use ratatui::Terminal;
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::embedder::{DownloadProgress, ModelPool};
use crate::metrics::LocalMetrics;

const TICK_RATE: Duration = Duration::from_millis(250);

/// Runs the local TUI dashboard until the user exits.
pub fn run_dashboard(metrics: Arc<LocalMetrics>, host: String, port: u16) -> CocoResult<()> {
    let _guard = TerminalGuard::enter()?;
    let mut terminal = setup_terminal()?;
    let mut sys = System::new();
    let pid = sysinfo::get_current_pid().ok();

    loop {
        let snapshot = metrics.snapshot();
        let memory_mb = memory_usage_mb(&mut sys, pid);
        let model_stats = ModelPool::global().and_then(|pool| pool.stats()).ok();

        terminal
            .draw(|frame| {
                let size = frame.area();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(7),
                        Constraint::Min(5),
                        Constraint::Length(1),
                    ])
                    .split(size);

                let header = Paragraph::new(Line::from(vec![
                    Span::styled("CoCo Local", Style::default().fg(Color::Cyan)),
                    Span::raw("  "),
                    Span::raw(format!("http://{}:{}", host, port)),
                ]))
                .block(Block::default().title("Status").borders(Borders::ALL));
                frame.render_widget(header, layout[0]);

                let mut metric_lines = Vec::new();
                metric_lines.push(Line::from(format!(
                    "Query QPS: {:.2} | Import QPS: {:.2}",
                    snapshot.query_qps, snapshot.import_qps
                )));
                metric_lines.push(Line::from(format!(
                    "Queries: {} | Imports: {}",
                    snapshot.queries_total, snapshot.imports_total
                )));
                metric_lines.push(Line::from(format!(
                    "Pending Events: {}",
                    snapshot.pending_events
                )));
                metric_lines.push(Line::from(match memory_mb {
                    Some(value) => format!("Memory: {:.1} MB", value),
                    None => "Memory: n/a".to_string(),
                }));
                if let Some(stats) = model_stats {
                    let mb = stats.total_bytes as f64 / (1024.0 * 1024.0);
                    metric_lines.push(Line::from(format!(
                        "Models Loaded: {} ({:.1} MB)",
                        stats.loaded_models, mb
                    )));
                }
                let metrics_block = Paragraph::new(metric_lines)
                    .block(Block::default().title("Metrics").borders(Borders::ALL));
                frame.render_widget(metrics_block, layout[1]);

                let recent_items: Vec<ListItem> = if snapshot.recent_files.is_empty() {
                    vec![ListItem::new("no recent files")]
                } else {
                    snapshot
                        .recent_files
                        .into_iter()
                        .map(ListItem::new)
                        .collect()
                };
                let recent = List::new(recent_items)
                    .block(Block::default().title("Recent Files").borders(Borders::ALL));
                frame.render_widget(recent, layout[2]);

                let footer = Paragraph::new(Line::from("Press q to quit"))
                    .block(Block::default().borders(Borders::NONE));
                frame.render_widget(footer, layout[3]);
            })
            .map_err(CocoError::system)?;

        if event::poll(TICK_RATE).map_err(CocoError::system)? {
            if let Event::Key(key) = event::read().map_err(CocoError::system)? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}

/// Runs a progress UI while downloading the default model.
pub fn run_model_download(
    progress: Arc<DownloadProgress>,
    model_name: String,
) -> CocoResult<()> {
    let _guard = TerminalGuard::enter()?;
    let mut terminal = setup_terminal()?;

    loop {
        let (downloaded, total, done) = progress.snapshot();
        let ratio = total
            .map(|total| downloaded as f64 / total as f64)
            .unwrap_or(0.0);
        let label = match total {
            Some(total) => format!(
                "{} / {} MB",
                format_mb(downloaded),
                format_mb(total)
            ),
            None => format!("{} MB", format_mb(downloaded)),
        };

        terminal
            .draw(|frame| {
                let size = frame.area();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ])
                    .split(size);

                let header = Paragraph::new(Line::from(vec![
                    Span::styled("CoCo Local", Style::default().fg(Color::Cyan)),
                    Span::raw("  "),
                    Span::raw(format!("Downloading {model_name}")),
                ]))
                .block(Block::default().title("Setup").borders(Borders::ALL));
                frame.render_widget(header, layout[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("Progress").borders(Borders::ALL))
                    .ratio(ratio.clamp(0.0, 1.0))
                    .label(label)
                    .gauge_style(Style::default().fg(Color::Green));
                frame.render_widget(gauge, layout[1]);

                let footer = Paragraph::new(Line::from(
                    "First-time model download. This may take a few minutes.",
                ));
                frame.render_widget(footer, layout[2]);
            })
            .map_err(CocoError::system)?;

        if done {
            break;
        }

        std::thread::sleep(TICK_RATE);
    }

    Ok(())
}

fn setup_terminal() -> CocoResult<Terminal<CrosstermBackend<Stdout>>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(CocoError::system)
}

fn memory_usage_mb(system: &mut System, pid: Option<Pid>) -> Option<f64> {
    let pid = pid?;
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    system
        .process(pid)
        .map(|process| process.memory() as f64 / 1024.0)
}

fn format_mb(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / (1024.0 * 1024.0))
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> CocoResult<Self> {
        enable_raw_mode().map_err(CocoError::system)?;
        execute!(io::stdout(), EnterAlternateScreen).map_err(CocoError::system)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
