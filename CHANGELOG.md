# Changelog

All notable changes to the Disk Cleanup Tool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Expanded Temporary Directory Detection**: Added 50+ more common temporary directories
  - Node.js: `.npm`, `.yarn`, `.pnpm-store`, `.turbo`, `.vite`, `.parcel-cache`, etc.
  - Python: `.pytest_cache`, `.mypy_cache`, `.tox`, `.ipynb_checkpoints`, etc.
  - Rust: `.fingerprint`, `.cargo`
  - Version managers: `.nvm`, `.rvm`, `.rbenv`, `.pyenv`
  - IDEs: `.idea`, `.vscode`, `.vs`, `.eclipse`
  - Coverage: `coverage`, `.nyc_output`, `htmlcov`
  - Build tools: `.gradle`, `.mvn`, `.webpack`, `.rollup.cache`
  - And many more (see README for full list)
  
- **Modern TUI with Ratatui**: Complete rewrite of interactive mode using the Ratatui library
  - Beautiful color-coded interface with icons (🗑 for temp dirs, 📁 for normal dirs)
  - Real-time statistics display (total size, selected count, space to be freed)
  - Smooth scrolling with automatic viewport adjustment
  - Visual selection indicators with checkboxes [✓]
  - Current item highlighting with background color
  
- **Enhanced Navigation**:
  - Vim-style navigation (j/k) in addition to arrow keys
  - Page navigation with PgUp/PgDn (jumps by 10 entries)
  - Home/End keys to jump to top/bottom
  - Esc key as alternative to 'q' for quitting
  
- **New Selection Features**:
  - 'a' key to select all directories (not just current page)
  - 'c' key to clear all selections
  - Visual feedback for selected items with green checkmarks
  
- **Live Scanning Progress**:
  - Animated spinner during filesystem scanning
  - Real-time file and directory count updates
  - Current path display (truncated if too long)
  - Clean, centered progress UI

### Changed
- **Interactive Mode**: Migrated from basic crossterm implementation to full Ratatui TUI
  - Replaced page-based navigation with smooth scrolling
  - Improved visual hierarchy and readability
  - Better use of terminal space with bordered sections
  - **Added 1 MB minimum size filter** to hide small directories and focus on significant space savings
  - Filter indicator shown in header: "(≥1 MB)"
  
- **Dependencies**: Already had ratatui 0.29 in Cargo.toml, now fully utilized

### Technical Details
- New module: `src/scan_ui.rs` for progress display during scanning
- Refactored `src/interactive.rs` to use Ratatui widgets and layouts
- Updated `src/main.rs` to use the new scan progress UI
- All existing tests still pass (30 tests)
- No breaking changes to CLI interface

### Documentation
- **Major README.md overhaul** - Consolidated all documentation into one comprehensive, catchy README
- Added FAQ section with common questions
- Improved visual hierarchy with better formatting and emojis
- Removed redundant documentation files (TUI_FEATURES.md, FILTER_RATIONALE.md, TEMP_DIRECTORIES.md, TESTING.md)
- All essential information now in main README for better discoverability

## [0.1.0] - Initial Release

### Added
- Core directory scanning functionality
- CSV export/import support
- Basic interactive mode
- Temporary directory detection
- Safe deletion with confirmation
- Command-line interface with clap
