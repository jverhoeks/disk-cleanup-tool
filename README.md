# 🧹 Disk Cleanup Tool

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-30%20passing-brightgreen.svg)](TESTING.md)

A blazingly fast Rust-based CLI tool with a beautiful TUI for analyzing and reclaiming disk space. Automatically identifies and removes temporary directories like `node_modules`, `.venv`, `target`, and 60+ more patterns across multiple ecosystems.

Built with [Ratatui](https://ratatui.rs/) for a smooth, responsive terminal experience.

## 📋 Table of Contents

- [Screenshots](#-screenshots)
- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Detected Patterns](#-detected-patterns-60)
- [Safety Features](#️-safety-features)
- [CSV Format](#-csv-format)
- [Testing](#-testing)
- [Architecture](#️-architecture)
- [Performance](#-performance)
- [Contributing](#-contributing)
- [Documentation](#-documentation)

## 📸 Screenshots

### Interactive Mode
```
┌─────────────────────────────────────────────────────────────────┐
│ Disk Cleanup Tool - Interactive Mode (≥1 MB)                   │
│ Total: 156 dirs | Size: 2.3 GB | Selected: 3 (450 MB)         │
└─────────────────────────────────────────────────────────────────┘
┌─────────── Directories (5/156) ────────────────────────────────┐
│ [ ] 🗑 /home/user/projects/app1/node_modules - 450 MB         │
│ [✓] 🗑 /home/user/projects/app2/.venv - 120 MB                │
│ [✓] 📁 /home/user/projects/app3/build - 89 MB                 │
│ [ ] 🗑 /home/user/projects/app4/target - 78 MB                │
│ [✓] 📁 /home/user/Downloads - 45 MB                           │
└─────────────────────────────────────────────────────────────────┘
```

### Live Scanning Progress
```
┌─────────────────────────────────────────────┐
│      🔍 Scanning Filesystem                 │
└─────────────────────────────────────────────┘
│  ⠋  Scanning directories...                │
│  Directories: 1,234  |  Files: 45,678       │
│  Current: /home/user/projects/my-app/...   │
└─────────────────────────────────────────────┘
```

## ✨ Features

- 🔍 **Smart scanning** - Recursive directory traversal with real-time progress
- 🎯 **60+ patterns** - Auto-detects temp directories across Node.js, Python, Rust, Java, and more
- 🖥️ **Beautiful TUI** - Modern interactive interface with colors, icons, and smooth navigation
- 📊 **CSV export/import** - Save scans for later review or analysis
- ⚡ **Live feedback** - Animated progress with file/directory counts and current path
- 🛡️ **Safe by design** - Explicit confirmation required, detailed previews, error resilience
- 🎨 **Smart filtering** - Interactive mode focuses on directories ≥1 MB for meaningful cleanup

## 📦 Installation

### From Source

```bash
git clone https://github.com/yourusername/disk-cleanup-tool.git
cd disk-cleanup-tool
cargo build --release
```

The binary will be available at `target/release/disk-cleanup-tool`.

### Requirements

- Rust 1.70+ (2021 edition)
- Works on macOS, Linux, and Windows

### Using Make (Optional)

```bash
make build    # Build release binary
make test     # Run tests
make install  # Install to ~/.cargo/bin
make help     # Show all available commands
```

## 🚀 Quick Start

### Basic Usage

```bash
# Scan current directory
disk-cleanup-tool

# Scan specific directory
disk-cleanup-tool --path ~/projects

# Show only temporary directories
disk-cleanup-tool --path ~/projects --temp-only

# Interactive mode - browse and delete
disk-cleanup-tool --path ~/projects --interactive
```

### 🎮 Interactive Mode

The interactive TUI provides a beautiful interface for reviewing and cleaning up directories:

```bash
disk-cleanup-tool --path ~/projects --temp-only --interactive
```

**Keyboard Controls:**

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate up/down |
| `PgUp/PgDn` | Jump 10 entries |
| `Home/End` | Jump to top/bottom |
| `Space` | Toggle selection |
| `a` | Select all |
| `c` | Clear selections |
| `d` | Delete selected (with confirmation) |
| `q` or `Esc` | Quit |

**Visual Features:**
- 🗑 Temp directories highlighted
- 📁 Normal directories
- [✓] Selection indicators
- Real-time stats (total size, selected count, space to free)
- Smooth scrolling and animations

**Smart Filtering:** Interactive mode shows only directories ≥1 MB to focus on meaningful cleanup (typically 96%+ of reclaimable space while hiding 92%+ of noise).

### 📊 CSV Export/Import

```bash
# Export scan results
disk-cleanup-tool --path ~/projects --output-csv results.csv

# Load and review later
disk-cleanup-tool --input-csv results.csv --interactive

# Combine operations
disk-cleanup-tool --path ~/projects --temp-only --output-csv temp.csv --interactive
```

### 💡 Common Workflows

**Clean up old projects:**
```bash
disk-cleanup-tool --path ~/old-projects --temp-only --interactive
```

**Find all node_modules:**
```bash
disk-cleanup-tool --path ~/projects --temp-only | grep node_modules
```

**Comprehensive analysis:**
```bash
# Scan and save everything
disk-cleanup-tool --path ~/projects --output-csv full_scan.csv

# Review CSV, then clean interactively
disk-cleanup-tool --input-csv full_scan.csv --temp-only --interactive
```

## 🎯 Detected Patterns (60+)

The tool uses exact name matching to identify temporary directories across multiple ecosystems:

<details>
<summary><b>Node.js / JavaScript</b> (15 patterns)</summary>

- `node_modules`, `bower_components` - Dependencies
- `.npm`, `.yarn`, `.pnpm-store` - Package manager caches
- `.next`, `.nuxt`, `.output` - Framework builds
- `.turbo`, `.parcel-cache`, `.webpack`, `.rollup.cache`, `.vite` - Build tool caches
- `.vercel`, `.netlify` - Deployment caches
</details>

<details>
<summary><b>Python</b> (11 patterns)</summary>

- `.venv`, `venv`, `env`, `.env` - Virtual environments
- `__pycache__`, `.pytest_cache`, `.mypy_cache` - Caches
- `.tox`, `.eggs`, `*.egg-info` - Testing/packaging
- `.ipynb_checkpoints` - Jupyter notebooks
</details>

<details>
<summary><b>Rust</b> (3 patterns)</summary>

- `target` - Build output
- `.fingerprint`, `.cargo` - Build artifacts
</details>

<details>
<summary><b>Build Outputs</b> (7 patterns)</summary>

- `dist`, `build`, `out`, `_build`, `.build` - Compiled outputs
- `.gradle`, `.mvn` - Java build tools
</details>

<details>
<summary><b>Caches & Temp</b> (8 patterns)</summary>

- `.cache`, `cache`, `.tmp`, `tmp`, `temp`, `.temp` - General
- `.sass-cache`, `.docusaurus` - Tool-specific
</details>

<details>
<summary><b>Version Managers</b> (4 patterns)</summary>

- `.nvm`, `.rvm`, `.rbenv`, `.pyenv`
</details>

<details>
<summary><b>IDEs & Editors</b> (5 patterns)</summary>

- `.idea`, `.vscode`, `.vs`, `.eclipse`, `.settings`
</details>

<details>
<summary><b>Testing & Coverage</b> (4 patterns)</summary>

- `coverage`, `.coverage`, `.nyc_output`, `htmlcov`
</details>

<details>
<summary><b>Operating System</b> (3 patterns)</summary>

- `.DS_Store`, `Thumbs.db`, `.Trash`
</details>

**Detection Strategy:** Exact name matching only (e.g., `node_modules` ✅, `my_node_modules` ❌) to prevent accidental deletion.

## 🛡️ Safety Features

- ✅ **Explicit confirmation** - Must type "yes" to confirm deletion
- 📋 **Detailed preview** - Shows all directories and total size before deletion
- 🔄 **Error resilience** - Continues batch deletion even if some operations fail
- 📊 **Clear reporting** - Shows success/failure status for each deletion
- 🎯 **Conservative matching** - Exact name matching prevents accidental deletion
- 💾 **CSV backup** - Export scans before cleanup for safety

## 📄 CSV Format

Exported CSV files contain:

| Column | Description |
|--------|-------------|
| `path` | Full directory path |
| `files` | Total file count (including subdirectories) |
| `size_bytes` | Total size in bytes |
| `type` | `"temp"` or `"normal"` |

Example:
```csv
path,files,size_bytes,type
/home/user/projects,150,2048576,normal
/home/user/projects/node_modules,5420,524288000,temp
```

## 🧪 Testing

Comprehensive test suite with 30 tests including property-based testing:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module
cargo test scanner::tests
```

**Test Coverage:**
- 12 unit tests for core functionality
- 18 property-based tests (using proptest) for correctness properties
- 100% coverage of all design properties
- Tests for scanning, CSV handling, deletion, and UI logic

## 🏗️ Architecture

- **Language:** Rust (2021 edition)
- **TUI Framework:** [Ratatui](https://ratatui.rs/) 0.29
- **Terminal Backend:** [Crossterm](https://github.com/crossterm-rs/crossterm) 0.28
- **CLI Parsing:** [Clap](https://github.com/clap-rs/clap) 4.5
- **Parallel Processing:** [Rayon](https://github.com/rayon-rs/rayon) 1.10
- **CSV Handling:** [csv](https://github.com/BurntSushi/rust-csv) 1.3

## 📈 Performance

- Fast parallel directory traversal
- Efficient size calculation with caching
- Minimal memory footprint
- Smooth 60 FPS UI rendering
- Handles thousands of directories with ease

## 🚀 Releasing

### Quick Start

```bash
# 1. Check if ready to release
./scripts/pre-release-check.sh

# 2. Create release (auto-generated notes)
./scripts/quick-release.sh

# OR use full release script with custom notes
./scripts/release.sh
```

### Automated CI/CD

Push a tag to trigger automated multi-platform builds:
```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will automatically build for Linux, macOS (x86_64 + ARM64), and Windows.

See [scripts/README.md](scripts/README.md) for detailed instructions.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Areas for improvement:**
- Additional temporary directory patterns
- Search/filter in interactive mode
- Directory tree view
- Configurable size threshold
- More export formats (JSON, etc.)

## 📝 License

MIT

## 🔗 Documentation

- [Changelog](CHANGELOG.md) - Version history and updates
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Testing Documentation](TESTING.md) - Detailed test coverage info
- [TUI Features](TUI_FEATURES.md) - Interactive mode details
- [Filter Rationale](FILTER_RATIONALE.md) - Why 1 MB minimum size
- [Temp Directories Reference](TEMP_DIRECTORIES.md) - Complete pattern list

---

**Made with ❤️ and Rust** | Star ⭐ if you find this useful!
