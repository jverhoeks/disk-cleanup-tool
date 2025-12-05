# TUI Features Documentation

This document provides detailed information about the Terminal User Interface (TUI) features and implementation.

## Overview

The Disk Cleanup Tool features a modern, responsive TUI built with [Ratatui](https://ratatui.rs/), providing a smooth and intuitive experience for managing disk space. The interface includes live scanning progress, interactive directory browsing, and comprehensive keyboard controls.

## Key Features

### 1. Live Scanning Progress

When scanning directories, you'll see:
```
┌─────────────────────────────────────────────┐
│      🔍 Scanning Filesystem                 │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│  ⠋  Scanning directories...                │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│  Directories: 1,234  |  Files: 45,678       │
└─────────────────────────────────────────────┘
┌──────────── Current Path ──────────────────┐
│  Current:                                   │
│  /home/user/projects/my-app/node_modules   │
└─────────────────────────────────────────────┘
```

Features:
- Animated spinner (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
- Real-time file and directory counts
- Current path being scanned
- Clean, centered layout

### 2. Interactive Directory Browser

```
┌─────────────────────────────────────────────────────────────────┐
│ Disk Cleanup Tool - Interactive Mode (≥1 MB)                   │
│ Total: 156 dirs | Size: 2.3 GB | Selected: 3 (450 MB)         │
└─────────────────────────────────────────────────────────────────┘
┌─────────── Directories (5/156) ────────────────────────────────┐
│ [ ] 🗑 /home/user/projects/app1/node_modules - 450 MB (...)   │
│ [✓] 🗑 /home/user/projects/app2/.venv - 120 MB (1,234 files)  │
│ [✓] 📁 /home/user/projects/app3/build - 89 MB (567 files)     │
│ [ ] 🗑 /home/user/projects/app4/target - 78 MB (890 files)    │
│ [✓] 📁 /home/user/Downloads - 45 MB (123 files)               │
└─────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────┐
│ ↑/↓ or j/k: Navigate | Space: Toggle | a: Select all | c: Clear│
│ PgUp/PgDn: Page | Home/End: Jump | d: Delete selected | q: Quit│
└─────────────────────────────────────────────────────────────────┘
```

Features:
- Color-coded entries (temp dirs in different color)
- Visual icons: 🗑 for temp directories, 📁 for normal
- Checkboxes [✓] for selected items
- Current item highlighted with background
- Real-time statistics in header
- Comprehensive keyboard shortcuts

### 3. Visual Indicators

#### Colors
- **Cyan**: Headers and titles
- **Yellow**: Size information and counts
- **Green**: Selected items and positive actions
- **Blue**: File counts
- **Red**: Quit/cancel actions
- **Gray/DarkGray**: Unselected items and secondary info
- **White**: Current/focused item

#### Icons
- 🔍 Scanning indicator
- 🗑 Temporary directory (node_modules, .venv, etc.)
- 📁 Normal directory
- [✓] Selected for deletion
- [ ] Not selected
- ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ Animated spinner frames

### 4. Keyboard Controls

#### Navigation
| Key | Action |
|-----|--------|
| `↑` / `k` | Move up one entry |
| `↓` / `j` | Move down one entry |
| `PgUp` | Jump up 10 entries |
| `PgDn` | Jump down 10 entries |
| `Home` | Jump to top |
| `End` | Jump to bottom |

#### Selection
| Key | Action |
|-----|--------|
| `Space` | Toggle current item |
| `a` | Select all directories |
| `c` | Clear all selections |

#### Actions
| Key | Action |
|-----|--------|
| `d` | Delete selected (with confirmation) |
| `q` / `Esc` | Quit without deleting |

### 5. Smart Scrolling

The TUI automatically adjusts the viewport to keep the current item visible:
- Scrolls down when navigating past the bottom
- Scrolls up when navigating past the top
- Maintains context by showing surrounding items
- Smooth, flicker-free rendering

### 6. Responsive Layout

The interface adapts to your terminal size:
- Automatically calculates visible items based on terminal height
- Truncates long paths intelligently
- Maintains readability at different terminal sizes
- Uses borders and spacing effectively

### 7. Smart Filtering

Interactive mode automatically filters directories to focus on significant space savings:
- **Minimum size: 1 MB** - Hides directories smaller than 1 MB
- Reduces clutter and focuses on directories worth cleaning
- Filter indicator shown in header: "(≥1 MB)"
- Non-interactive mode still shows all directories for comprehensive analysis

## Technical Implementation

### Architecture
- **Backend**: CrosstermBackend for cross-platform terminal control
- **Framework**: Ratatui for widget rendering and layout
- **Event Handling**: Crossterm for keyboard input
- **Terminal Modes**: Raw mode with alternate screen buffer

### Performance
- Efficient rendering with minimal redraws
- Non-blocking event polling (100ms timeout)
- Smooth animations at ~12 FPS
- Instant response to keyboard input

### Error Handling
- Graceful terminal restoration on errors
- Proper cleanup of raw mode and alternate screen
- Clear error messages if terminal operations fail

## Comparison: Before vs After

### Before (Basic Crossterm)
- Simple text-based output with `println!`
- Page-based navigation (25 items per page)
- Basic cursor positioning
- Limited visual feedback
- Manual screen clearing

### After (Ratatui TUI)
- Rich, color-coded interface with borders
- Smooth scrolling through all items
- Automatic viewport management
- Comprehensive visual indicators
- Professional terminal UI framework

## Future Enhancements

Potential improvements:
- Search/filter functionality
- Sort options (by size, name, type)
- Directory tree view
- Detailed info panel for selected item
- Batch operations (select by pattern)
- Undo/redo for selections
- Export selection list
