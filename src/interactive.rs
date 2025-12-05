use crate::scanner::{DirectoryEntry, EntryType};
use crate::utils::format_size;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum InteractiveError {
    #[error("Terminal error: {0}")]
    TerminalError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct InteractiveSession {
    entries: Vec<DirectoryEntry>,
    selected: HashSet<usize>,
    current_index: usize,
    scroll_offset: usize,
}

impl InteractiveSession {
    pub fn new(mut entries: Vec<DirectoryEntry>) -> Self {
        const MIN_SIZE_BYTES: u64 = 1024 * 1024; // 1 MB

        // Filter out directories smaller than 1MB
        entries.retain(|e| e.cumulative_size_bytes >= MIN_SIZE_BYTES);

        // Sort by cumulative size descending
        entries.sort_by(|a, b| b.cumulative_size_bytes.cmp(&a.cumulative_size_bytes));

        Self {
            entries,
            selected: HashSet::new(),
            current_index: 0,
            scroll_offset: 0,
        }
    }

    pub fn run(&mut self) -> Result<Vec<PathBuf>, InteractiveError> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<Vec<PathBuf>, InteractiveError> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                return Ok(Vec::new());
                            }
                            KeyCode::Char(' ') => {
                                self.toggle_selection();
                            }
                            KeyCode::Char('d') | KeyCode::Char('D') => {
                                if !self.selected.is_empty() {
                                    return Ok(self.get_selected_paths());
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.move_up();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.move_down();
                            }
                            KeyCode::Char('a') | KeyCode::Char('A') => {
                                self.select_all_visible();
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                self.clear_all_selections();
                            }
                            KeyCode::PageUp => {
                                self.page_up();
                            }
                            KeyCode::PageDown => {
                                self.page_down();
                            }
                            KeyCode::Home => {
                                self.go_to_top();
                            }
                            KeyCode::End => {
                                self.go_to_bottom();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // List
                Constraint::Length(4),  // Footer
            ])
            .split(f.area());

        self.render_header(f, chunks[0]);
        self.render_list(f, chunks[1]);
        self.render_footer(f, chunks[2]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let total_size: u64 = self.entries.iter().map(|e| e.cumulative_size_bytes).sum();
        let selected_size: u64 = self.selected.iter()
            .filter_map(|&idx| self.entries.get(idx))
            .map(|e| e.cumulative_size_bytes)
            .sum();

        let header_text = vec![
            Line::from(vec![
                Span::styled("Disk Cleanup Tool", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" - Interactive Mode "),
                Span::styled("(≥1 MB)", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::raw("Total: "),
                Span::styled(format!("{} dirs", self.entries.len()), Style::default().fg(Color::Yellow)),
                Span::raw(" | Size: "),
                Span::styled(format_size(total_size), Style::default().fg(Color::Yellow)),
                Span::raw(" | Selected: "),
                Span::styled(format!("{}", self.selected.len()), Style::default().fg(Color::Green)),
                Span::raw(" ("),
                Span::styled(format_size(selected_size), Style::default().fg(Color::Green)),
                Span::raw(")"),
            ]),
        ];

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
        f.render_widget(header, area);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let list_height = area.height.saturating_sub(2) as usize; // Account for borders
        
        // Adjust scroll offset to keep current item visible
        if self.current_index < self.scroll_offset {
            self.scroll_offset = self.current_index;
        } else if self.current_index >= self.scroll_offset + list_height {
            self.scroll_offset = self.current_index.saturating_sub(list_height - 1);
        }

        let visible_entries: Vec<ListItem> = self.entries
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(list_height)
            .map(|(idx, entry)| {
                let is_selected = self.selected.contains(&idx);
                let is_current = idx == self.current_index;
                
                let checkbox = if is_selected { "[✓]" } else { "[ ]" };
                let type_marker = match entry.entry_type {
                    EntryType::Temp => "🗑 ",
                    EntryType::Normal => "📁 ",
                };

                let path_str = entry.path.display().to_string();
                let size_str = format_size(entry.cumulative_size_bytes);
                let files_str = format!("{} files", entry.cumulative_file_count);

                let line = vec![
                    Span::styled(checkbox.to_string(), if is_selected { 
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) 
                    } else { 
                        Style::default().fg(Color::DarkGray) 
                    }),
                    Span::raw(" "),
                    Span::raw(type_marker.to_string()),
                    Span::styled(path_str, if is_current {
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    }),
                    Span::raw(" - "),
                    Span::styled(size_str, Style::default().fg(Color::Yellow)),
                    Span::raw(" ("),
                    Span::styled(files_str, Style::default().fg(Color::Blue)),
                    Span::raw(")"),
                ];

                let item = ListItem::new(Line::from(line));
                if is_current {
                    item.style(Style::default().bg(Color::DarkGray))
                } else {
                    item
                }
            })
            .collect();

        let list = List::new(visible_entries)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(format!(" Directories ({}/{}) ", self.current_index + 1, self.entries.len())));

        f.render_widget(list, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_text = vec![
            Line::from(vec![
                Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
                Span::raw(" or "),
                Span::styled("j/k", Style::default().fg(Color::Cyan)),
                Span::raw(": Navigate | "),
                Span::styled("Space", Style::default().fg(Color::Cyan)),
                Span::raw(": Toggle | "),
                Span::styled("a", Style::default().fg(Color::Cyan)),
                Span::raw(": Select all | "),
                Span::styled("c", Style::default().fg(Color::Cyan)),
                Span::raw(": Clear"),
            ]),
            Line::from(vec![
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Cyan)),
                Span::raw(": Page | "),
                Span::styled("Home/End", Style::default().fg(Color::Cyan)),
                Span::raw(": Jump | "),
                Span::styled("d", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(": Delete selected | "),
                Span::styled("q/Esc", Style::default().fg(Color::Red)),
                Span::raw(": Quit"),
            ]),
        ];

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));
        f.render_widget(footer, area);
    }

    fn toggle_selection(&mut self) {
        if self.current_index < self.entries.len() {
            if self.selected.contains(&self.current_index) {
                self.selected.remove(&self.current_index);
            } else {
                self.selected.insert(self.current_index);
            }
        }
    }

    fn select_all_visible(&mut self) {
        for i in 0..self.entries.len() {
            self.selected.insert(i);
        }
    }

    fn clear_all_selections(&mut self) {
        self.selected.clear();
    }

    fn move_up(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.current_index + 1 < self.entries.len() {
            self.current_index += 1;
        }
    }

    fn page_up(&mut self) {
        self.current_index = self.current_index.saturating_sub(10);
    }

    fn page_down(&mut self) {
        self.current_index = (self.current_index + 10).min(self.entries.len().saturating_sub(1));
    }

    fn go_to_top(&mut self) {
        self.current_index = 0;
        self.scroll_offset = 0;
    }

    fn go_to_bottom(&mut self) {
        self.current_index = self.entries.len().saturating_sub(1);
    }

    fn get_selected_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for &idx in &self.selected {
            if idx < self.entries.len() {
                paths.push(self.entries[idx].path.clone());
            }
        }
        paths
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use crate::scanner::EntryType;
    use proptest::prelude::*;
    use std::path::PathBuf;

    // Feature: disk-cleanup-tool, Property 12: Top N sorting
    // Validates: Requirements 4.1
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_entries_sorted_by_size(
            sizes in prop::collection::vec(1048576u64..100000000, 2..20)  // 1MB to 100MB
        ) {
            let mut entries = Vec::new();
            for (i, size) in sizes.iter().enumerate() {
                entries.push(DirectoryEntry {
                    path: PathBuf::from(format!("/dir{}", i)),
                    file_count: 1,
                    size_bytes: *size,
                    cumulative_file_count: 1,
                    cumulative_size_bytes: *size,
                    entry_type: EntryType::Normal,
                });
            }

            let session = InteractiveSession::new(entries);

            // Verify entries are sorted by cumulative size descending
            for i in 0..session.entries.len() - 1 {
                prop_assert!(session.entries[i].cumulative_size_bytes >= session.entries[i + 1].cumulative_size_bytes);
            }
        }

        // Feature: disk-cleanup-tool, Property 14: Selection toggle
        // Validates: Requirements 4.3
        #[test]
        fn test_selection_toggle(num_entries in 2usize..10, toggle_idx in 0usize..5) {
            const MIN_SIZE: u64 = 1024 * 1024; // 1 MB
            let mut entries = Vec::new();
            for i in 0..num_entries {
                entries.push(DirectoryEntry {
                    path: PathBuf::from(format!("/dir{}", i)),
                    file_count: 1,
                    size_bytes: MIN_SIZE,
                    cumulative_file_count: 1,
                    cumulative_size_bytes: MIN_SIZE,
                    entry_type: EntryType::Normal,
                });
            }

            let mut session = InteractiveSession::new(entries);
            
            // Session should have all entries since they're all >= 1MB
            prop_assert_eq!(session.entries.len(), num_entries);
            
            let idx = toggle_idx % num_entries;
            session.current_index = idx;

            // Initially not selected
            prop_assert!(!session.selected.contains(&idx));

            // Toggle on
            session.toggle_selection();
            prop_assert!(session.selected.contains(&idx));

            // Toggle off
            session.toggle_selection();
            prop_assert!(!session.selected.contains(&idx));
        }
    }
}
