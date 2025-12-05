use crate::scanner::{DirectoryEntry, EntryType};
use crate::utils::format_size;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::path::PathBuf;

pub enum SummaryAction {
    Continue,
    LaunchInteractive,
}

pub fn show_summary(entries: &[DirectoryEntry], root_path: &PathBuf) -> io::Result<SummaryAction> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_summary_ui(&mut terminal, entries, root_path);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_summary_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    entries: &[DirectoryEntry],
    root_path: &PathBuf,
) -> io::Result<SummaryAction> {
    let mut scroll_offset = 0usize;
    
    loop {
        terminal.draw(|f| {
            render_summary(f, entries, root_path, scroll_offset);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                        return Ok(SummaryAction::Continue);
                    }
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        return Ok(SummaryAction::LaunchInteractive);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        scroll_offset = scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        scroll_offset = scroll_offset.saturating_add(1).min(entries.len().saturating_sub(1));
                    }
                    KeyCode::PageUp => {
                        scroll_offset = scroll_offset.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        scroll_offset = scroll_offset.saturating_add(10).min(entries.len().saturating_sub(1));
                    }
                    KeyCode::Home => {
                        scroll_offset = 0;
                    }
                    KeyCode::End => {
                        scroll_offset = entries.len().saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render_summary(f: &mut Frame, entries: &[DirectoryEntry], root_path: &PathBuf, scroll_offset: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Header with stats
            Constraint::Min(0),     // Top directories list
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Calculate stats
    let root_entry = entries.iter().find(|e| &e.path == root_path);
    let temp_count = entries.iter().filter(|e| matches!(e.entry_type, EntryType::Temp)).count();
    let temp_size: u64 = entries.iter()
        .filter(|e| matches!(e.entry_type, EntryType::Temp))
        .map(|e| e.cumulative_size_bytes)
        .sum();

    // Header
    let header_lines = if let Some(root) = root_entry {
        vec![
            Line::from(vec![
                Span::styled("📊 Scan Summary", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Root: "),
                Span::styled(root_path.display().to_string(), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("Total directories: "),
                Span::styled(format!("{}", entries.len()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("  |  Files: "),
                Span::styled(format!("{}", root.cumulative_file_count), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::raw("  |  Size: "),
                Span::styled(format_size(root.cumulative_size_bytes), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("Temp directories: "),
                Span::styled(format!("{}", temp_count), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw("  |  Temp size: "),
                Span::styled(format_size(temp_size), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("📊 Scan Summary", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Total directories: "),
                Span::styled(format!("{}", entries.len()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("Temp directories: "),
                Span::styled(format!("{}", temp_count), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw("  |  Temp size: "),
                Span::styled(format_size(temp_size), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ]),
        ]
    };

    let header = Paragraph::new(header_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(header, chunks[0]);

    // Top directories list
    let list_height = chunks[1].height.saturating_sub(2) as usize;
    let display_count = 20.min(entries.len());
    
    let items: Vec<ListItem> = entries
        .iter()
        .take(display_count)
        .skip(scroll_offset)
        .take(list_height)
        .enumerate()
        .map(|(idx, entry)| {
            let type_marker = match entry.entry_type {
                EntryType::Temp => "🗑 ",
                EntryType::Normal => "📁 ",
            };
            
            let rank = scroll_offset + idx + 1;
            
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:2}. ", rank), Style::default().fg(Color::DarkGray)),
                Span::raw(type_marker),
                Span::styled(
                    entry.path.display().to_string(),
                    if matches!(entry.entry_type, EntryType::Temp) {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::White)
                    }
                ),
                Span::raw(" - "),
                Span::styled(format_size(entry.cumulative_size_bytes), Style::default().fg(Color::Yellow)),
                Span::raw(" ("),
                Span::styled(format!("{} files", entry.cumulative_file_count), Style::default().fg(Color::Blue)),
                Span::raw(")"),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(format!(" Top {} Largest Directories ", display_count)));
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" or "),
            Span::styled("j/k", Style::default().fg(Color::Cyan)),
            Span::raw(": Scroll  |  "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Cyan)),
            Span::raw(": Page  |  "),
            Span::styled("i", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Interactive mode  |  "),
            Span::styled("q", Style::default().fg(Color::Green)),
            Span::raw(": Exit"),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));
    f.render_widget(footer, chunks[2]);
}
