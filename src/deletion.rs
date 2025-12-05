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
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DeletionError {
    #[error("Permission denied for {path}")]
    PermissionDenied { path: PathBuf },

    #[error("Failed to delete {path}: {reason}")]
    DeletionFailed { path: PathBuf, reason: String },
}

pub struct DeletionReport {
    pub successful: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
    pub total_freed_bytes: u64,
}

impl DeletionReport {
    pub fn show_report(&self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = run_report_ui(&mut terminal, self);

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }
}

fn run_report_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    report: &DeletionReport,
) -> io::Result<()> {
    let mut scroll_offset = 0usize;
    
    loop {
        terminal.draw(|f| {
            render_report(f, report, scroll_offset);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                        return Ok(());
                    }
                    KeyCode::Up => {
                        scroll_offset = scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        let total_items = report.successful.len() + report.failed.len();
                        scroll_offset = scroll_offset.saturating_add(1).min(total_items.saturating_sub(1));
                    }
                    KeyCode::PageUp => {
                        scroll_offset = scroll_offset.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        let total_items = report.successful.len() + report.failed.len();
                        scroll_offset = scroll_offset.saturating_add(10).min(total_items.saturating_sub(1));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render_report(f: &mut Frame, report: &DeletionReport, scroll_offset: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // Header
            Constraint::Min(0),     // List
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Header
    let success_color = if report.failed.is_empty() { Color::Green } else { Color::Yellow };
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("✓ Deletion Complete", Style::default().fg(success_color).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Successfully deleted: "),
            Span::styled(format!("{}", report.successful.len()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Failed: "),
            Span::styled(format!("{}", report.failed.len()), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("  |  Space freed: "),
            Span::styled(format_size(report.total_freed_bytes), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(success_color)));
    f.render_widget(header, chunks[0]);

    // List of results
    let list_height = chunks[1].height.saturating_sub(2) as usize;
    let mut items = Vec::new();

    // Add successful deletions
    for path in &report.successful {
        items.push((true, path.clone(), String::new()));
    }

    // Add failed deletions
    for (path, reason) in &report.failed {
        items.push((false, path.clone(), reason.clone()));
    }

    let list_items: Vec<ListItem> = items
        .iter()
        .skip(scroll_offset)
        .take(list_height)
        .map(|(success, path, reason)| {
            if *success {
                ListItem::new(Line::from(vec![
                    Span::styled("  ✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(path.display().to_string(), Style::default().fg(Color::White)),
                ]))
            } else {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled("  ✗ ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                        Span::styled(path.display().to_string(), Style::default().fg(Color::Red)),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(reason.clone(), Style::default().fg(Color::DarkGray)),
                    ]),
                ])
            }
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(format!(" Results ({}/{}) ", scroll_offset + 1, items.len())));
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(": Scroll  |  "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Cyan)),
            Span::raw(": Page  |  "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(" or "),
            Span::styled("q", Style::default().fg(Color::Green)),
            Span::raw(": Close"),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));
    f.render_widget(footer, chunks[2]);
}

pub fn confirm_deletion(paths: &[PathBuf]) -> bool {
    if paths.is_empty() {
        return false;
    }

    // Calculate total size
    let mut total_size = 0u64;
    for path in paths {
        if let Ok(size) = calculate_dir_size(path) {
            total_size += size;
        }
    }

    // Setup terminal
    if let Err(_) = enable_raw_mode() {
        return fallback_confirm_deletion(paths, total_size);
    }
    
    let mut stdout = io::stdout();
    if let Err(_) = execute!(stdout, EnterAlternateScreen) {
        let _ = disable_raw_mode();
        return fallback_confirm_deletion(paths, total_size);
    }
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(_) => {
            let _ = disable_raw_mode();
            return fallback_confirm_deletion(paths, total_size);
        }
    };

    let result = run_confirmation_ui(&mut terminal, paths, total_size);

    // Restore terminal
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    result.unwrap_or(false)
}

fn fallback_confirm_deletion(paths: &[PathBuf], total_size: u64) -> bool {
    println!("\n=== DELETION CONFIRMATION ===");
    println!("You are about to delete {} directories:", paths.len());
    for path in paths {
        println!("  - {}", path.display());
    }
    println!("\nTotal size to be freed: {}", format_size(total_size));
    println!("\nThis action cannot be undone!");
    print!("Type 'yes' to confirm deletion: ");
    use std::io::Write;
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim() == "yes"
}

fn run_confirmation_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    paths: &[PathBuf],
    total_size: u64,
) -> io::Result<bool> {
    let mut scroll_offset = 0usize;
    
    loop {
        terminal.draw(|f| {
            render_confirmation(f, paths, total_size, scroll_offset);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    KeyCode::Up => {
                        scroll_offset = scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        scroll_offset = scroll_offset.saturating_add(1).min(paths.len().saturating_sub(1));
                    }
                    KeyCode::PageUp => {
                        scroll_offset = scroll_offset.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        scroll_offset = scroll_offset.saturating_add(10).min(paths.len().saturating_sub(1));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render_confirmation(f: &mut Frame, paths: &[PathBuf], total_size: u64, scroll_offset: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header
            Constraint::Min(0),     // List
            Constraint::Length(6),  // Footer
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("⚠️  DELETION CONFIRMATION", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Directories to delete: "),
            Span::styled(format!("{}", paths.len()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Total size to be freed: "),
            Span::styled(format_size(total_size), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Red)));
    f.render_widget(header, chunks[0]);

    // List of paths
    let list_height = chunks[1].height.saturating_sub(2) as usize;
    let items: Vec<ListItem> = paths
        .iter()
        .skip(scroll_offset)
        .take(list_height)
        .map(|path| {
            ListItem::new(Line::from(vec![
                Span::raw("  🗑  "),
                Span::styled(path.display().to_string(), Style::default().fg(Color::White)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(format!(" Directories ({}/{}) ", scroll_offset + 1, paths.len())));
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("⚠️  THIS ACTION CANNOT BE UNDONE!", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Confirm deletion  |  "),
            Span::styled("N", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" / "),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Cancel"),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));
    f.render_widget(footer, chunks[2]);
}

pub fn delete_directories(paths: &[PathBuf]) -> Result<DeletionReport, DeletionError> {
    let mut report = DeletionReport {
        successful: Vec::new(),
        failed: Vec::new(),
        total_freed_bytes: 0,
    };

    for path in paths {
        // Calculate size before deletion
        let size = calculate_dir_size(path).unwrap_or(0);

        match fs::remove_dir_all(path) {
            Ok(_) => {
                report.successful.push(path.clone());
                report.total_freed_bytes += size;
                println!("✓ Deleted: {}", path.display());
            }
            Err(e) => {
                let reason = e.to_string();
                report.failed.push((path.clone(), reason.clone()));
                eprintln!("✗ Failed to delete {}: {}", path.display(), reason);
            }
        }
    }

    Ok(report)
}

fn calculate_dir_size(path: &PathBuf) -> io::Result<u64> {
    let mut total = 0u64;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total += metadata.len();
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_delete_directories() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test directories
        let dir1 = root.join("dir1");
        let dir2 = root.join("dir2");
        fs::create_dir(&dir1).unwrap();
        fs::create_dir(&dir2).unwrap();
        fs::write(dir1.join("file.txt"), "content").unwrap();
        fs::write(dir2.join("file.txt"), "content").unwrap();

        let paths = vec![dir1.clone(), dir2.clone()];

        let report = delete_directories(&paths).unwrap();

        assert_eq!(report.successful.len(), 2);
        assert_eq!(report.failed.len(), 0);
        assert!(report.total_freed_bytes > 0);
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }

    #[test]
    fn test_delete_nonexistent_directory() {
        let paths = vec![PathBuf::from("/nonexistent/path")];

        let report = delete_directories(&paths).unwrap();

        assert_eq!(report.successful.len(), 0);
        assert_eq!(report.failed.len(), 1);
    }

    #[test]
    fn test_calculate_dir_size() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        fs::write(root.join("file1.txt"), "hello").unwrap();
        fs::write(root.join("file2.txt"), "world").unwrap();

        let size = calculate_dir_size(&root.to_path_buf()).unwrap();
        assert_eq!(size, 10); // "hello" + "world"
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::TempDir;

    // Feature: disk-cleanup-tool, Property 17: Deletion execution
    // Validates: Requirements 5.4
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn test_delete_directories_removes_all(
            num_dirs in 1usize..5
        ) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            let mut paths = Vec::new();
            for i in 0..num_dirs {
                let dir_path = root.join(format!("dir{}", i));
                fs::create_dir(&dir_path).unwrap();
                fs::write(dir_path.join("file.txt"), "content").unwrap();
                paths.push(dir_path);
            }

            // All directories should exist
            for path in &paths {
                prop_assert!(path.exists());
            }

            let report = delete_directories(&paths).unwrap();

            // All should be deleted
            prop_assert_eq!(report.successful.len(), num_dirs);
            prop_assert_eq!(report.failed.len(), 0);

            // Verify they're gone
            for path in &paths {
                prop_assert!(!path.exists());
            }
        }

        // Feature: disk-cleanup-tool, Property 26: Batch deletion resilience
        // Validates: Requirements 9.2
        #[test]
        fn test_deletion_continues_on_error(
            num_good in 1usize..3
        ) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            let mut paths = Vec::new();
            
            // Create some valid directories
            for i in 0..num_good {
                let dir_path = root.join(format!("good{}", i));
                fs::create_dir(&dir_path).unwrap();
                paths.push(dir_path);
            }

            // Add a nonexistent path
            paths.push(PathBuf::from("/nonexistent/path"));

            let report = delete_directories(&paths).unwrap();

            // Should have some successes and some failures
            prop_assert!(report.successful.len() > 0);
            prop_assert!(report.failed.len() > 0);
            prop_assert_eq!(report.successful.len() + report.failed.len(), paths.len());
        }
    }
}
