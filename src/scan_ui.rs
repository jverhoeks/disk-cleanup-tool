use crate::scanner::{DirectoryEntry, ScanConfig};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ScanProgress {
    pub files_scanned: u64,
    pub dirs_scanned: u64,
    pub current_path: String,
}

impl ScanProgress {
    pub fn new() -> Self {
        Self {
            files_scanned: 0,
            dirs_scanned: 0,
            current_path: String::new(),
        }
    }
}

pub fn scan_with_progress(config: ScanConfig) -> Result<Vec<DirectoryEntry>, Box<dyn std::error::Error>> {
    let progress = Arc::new(Mutex::new(ScanProgress::new()));
    let progress_clone = Arc::clone(&progress);

    // Spawn scanning thread
    let scan_handle = thread::spawn(move || {
        crate::scanner::scan_directory(config)
    });

    // Setup terminal for progress display
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Progress display loop
    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut frame_idx = 0;

    loop {
        if scan_handle.is_finished() {
            break;
        }

        terminal.draw(|f| {
            render_scan_progress(f, &progress_clone, spinner_frames[frame_idx]);
        })?;

        frame_idx = (frame_idx + 1) % spinner_frames.len();
        thread::sleep(Duration::from_millis(80));
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Get scan result
    let result = scan_handle.join().map_err(|_| "Scan thread panicked")??;
    
    Ok(result)
}

fn render_scan_progress(f: &mut Frame, progress: &Arc<Mutex<ScanProgress>>, spinner: &str) {
    let prog = progress.lock().unwrap();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("🔍 Scanning Filesystem", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, chunks[0]);

    // Spinner and status
    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(spinner, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  Scanning directories..."),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);

    // Stats
    let stats = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Directories: "),
            Span::styled(format!("{}", prog.dirs_scanned), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  |  Files: "),
            Span::styled(format!("{}", prog.files_scanned), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[2]);

    // Current path
    let path_display = if prog.current_path.len() > 60 {
        format!("...{}", &prog.current_path[prog.current_path.len() - 57..])
    } else {
        prog.current_path.clone()
    };

    let current = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Current: ", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(path_display, Style::default().fg(Color::Gray)),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title(" Current Path "));
    f.render_widget(current, chunks[3]);
}
