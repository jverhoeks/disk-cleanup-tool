# 🧹 Disk Cleanup Tool

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-30%20passing-brightgreen.svg)](#testing)

> **Reclaim gigabytes in seconds.** A blazingly fast Rust CLI with a beautiful TUI for analyzing and cleaning disk space. Automatically detects 60+ temporary directory patterns across Node.js, Python, Rust, Java, and more.

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

## ✨ Why Use This?

- 🚀 **Fast** - Parallel scanning with real-time progress
- 🎯 **Smart** - Detects 60+ patterns: `node_modules`, `.venv`, `target`, caches, and more
- 🖥️ **Beautiful** - Modern TUI with colors, icons, vim-style navigation
- 🛡️ **Safe** - Explicit confirmation, detailed previews, error resilience
- 📊 **Flexible** - CSV export/import for analysis and batch operations
- 🎨 **Focused** - Smart 1 MB filter shows only directories worth cleaning

## 📦 Installation

```bash
# Clone and build
git clone https://github.com/yourusername/disk-cleanup-tool.git
cd disk-cleanup-tool
cargo build --release

# Or use Make
make build
make install  # Installs to ~/.cargo/bin
```

**Requirements:** Rust 1.70+ | Works on macOS, Linux, Windows

## 🚀 Quick Start

```bash
# Scan and clean interactively
disk-cleanup-tool --path ~/projects --interactive

# Show only temp directories
disk-cleanup-tool --path ~/projects --temp-only

# Export to CSV for later
disk-cleanup-tool --path ~/projects --output-csv scan.csv
```

## 🎮 Interactive Mode

Launch the beautiful TUI to browse, select, and delete directories:

```bash
disk-cleanup-tool --path ~/projects --temp-only --interactive
```

### Keyboard Controls

| Key | Action | Key | Action |
|-----|--------|-----|--------|
| `↑/↓` `j/k` | Navigate | `Space` | Toggle selection |
| `PgUp/PgDn` | Jump 10 | `a` | Select all |
| `Home/End` | Jump to top/bottom | `c` | Clear all |
| `d` | Delete selected | `q` `Esc` | Quit |

### Features

- 🗑 **Color-coded** - Temp dirs highlighted, normal dirs in different color
- [✓] **Visual selection** - Checkboxes show what's selected
- 📊 **Real-time stats** - Total size, selected count, space to free
- ⚡ **Smooth scrolling** - Responsive navigation through thousands of entries
- 🎯 **Smart filter** - Shows only dirs ≥1 MB (hides 92% of noise, keeps 96%+ of reclaimable space)

## 🎯 What Gets Detected?

**60+ patterns across 10+ ecosystems** using exact name matching:

### Node.js / JavaScript (15)
`node_modules` • `bower_components` • `.npm` • `.yarn` • `.pnpm-store` • `.next` • `.nuxt` • `.output` • `.turbo` • `.parcel-cache` • `.webpack` • `.rollup.cache` • `.vite` • `.vercel` • `.netlify`

### Python (11)
`.venv` • `venv` • `env` • `.env` • `__pycache__` • `.pytest_cache` • `.mypy_cache` • `.tox` • `.eggs` • `*.egg-info` • `.ipynb_checkpoints`

### Rust (3)
`target` • `.fingerprint` • `.cargo`

### Build Outputs (7)
`dist` • `build` • `out` • `_build` • `.build` • `.gradle` • `.mvn`

### Caches (8)
`.cache` • `cache` • `.tmp` • `tmp` • `temp` • `.temp` • `.sass-cache` • `.docusaurus`

### Version Managers (4)
`.nvm` • `.rvm` • `.rbenv` • `.pyenv`

### IDEs (5)
`.idea` • `.vscode` • `.vs` • `.eclipse` • `.settings`

### Testing (4)
`coverage` • `.coverage` • `.nyc_output` • `htmlcov`

### OS (3)
`.DS_Store` • `Thumbs.db` • `.Trash`

**Detection Strategy:** Exact name matching only (`node_modules` ✅ | `my_node_modules` ❌) prevents accidental deletion.

## 💡 Common Workflows

### Clean up old projects
```bash
disk-cleanup-tool --path ~/old-projects --temp-only --interactive
```

### Find all node_modules
```bash
disk-cleanup-tool --path ~/projects --temp-only | grep node_modules
```

### Scan now, clean later
```bash
# Export scan results
disk-cleanup-tool --path ~/projects --output-csv scan.csv

# Review CSV, then clean interactively
disk-cleanup-tool --input-csv scan.csv --temp-only --interactive
```

### Comprehensive analysis
```bash
# Full scan with all directories
disk-cleanup-tool --path ~/projects --output-csv full_scan.csv

# Filter and clean specific types
disk-cleanup-tool --input-csv full_scan.csv --interactive
```

## 📊 CSV Export/Import

Save scans for later review or batch processing:

```bash
# Export
disk-cleanup-tool --path ~/projects --output-csv results.csv

# Import and clean
disk-cleanup-tool --input-csv results.csv --interactive
```

**CSV Format:**
```csv
path,files,size_bytes,type
/home/user/projects,150,2048576,normal
/home/user/projects/node_modules,5420,524288000,temp
```

## 🛡️ Safety Features

- ✅ **Explicit confirmation** - Must type "yes" to delete
- 📋 **Detailed preview** - Shows all directories and total size
- 🔄 **Error resilience** - Continues if some deletions fail
- 📊 **Clear reporting** - Success/failure status for each operation
- 🎯 **Conservative matching** - Exact names only, no wildcards
- 💾 **CSV backup** - Export before cleanup for safety

## 🧪 Testing

**30 tests** with 100% property coverage:

```bash
cargo test              # Run all tests
make test              # Using Make
cargo test -- --nocapture  # With output
```

**Coverage:**
- 12 unit tests for core functionality
- 18 property-based tests (proptest) for correctness
- Tests for scanning, CSV, deletion, and UI logic

## 🏗️ Architecture

Built with modern Rust tools:

- **[Ratatui](https://ratatui.rs/)** 0.29 - Beautiful TUI framework
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** 0.28 - Cross-platform terminal
- **[Clap](https://github.com/clap-rs/clap)** 4.5 - CLI parsing
- **[Rayon](https://github.com/rayon-rs/rayon)** 1.10 - Parallel processing
- **[csv](https://github.com/BurntSushi/rust-csv)** 1.3 - CSV handling
- **[proptest](https://github.com/proptest-rs/proptest)** 1.5 - Property-based testing

## 📈 Performance

- ⚡ Parallel directory traversal
- 💾 Efficient size calculation with caching
- 🪶 Minimal memory footprint
- 🎬 Smooth 60 FPS UI rendering
- 📦 Handles thousands of directories effortlessly

## 🚀 Releasing

### Manual Release

```bash
./scripts/pre-release-check.sh  # Validate everything
./scripts/quick-release.sh      # Create GitHub release
```

### Automated CI/CD

Push a tag to trigger multi-platform builds:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions automatically builds for:
- Linux x86_64
- macOS x86_64 + ARM64 (Apple Silicon)
- Windows x86_64

See [scripts/README.md](scripts/README.md) for details.

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Ideas for improvement:**
- Additional temp directory patterns
- Search/filter in interactive mode
- Directory tree view
- Configurable size threshold (`--min-size` flag)
- More export formats (JSON, YAML)
- Undo/redo for selections

## 📚 Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [scripts/README.md](scripts/README.md) - Release scripts documentation

## ❓ FAQ

<details>
<summary><b>Why does interactive mode only show directories ≥1 MB?</b></summary>

To focus on meaningful cleanup. In typical projects:
- 92% of directories are < 1 MB (only 4% of total space)
- 8% of directories are ≥ 1 MB (96% of total space)

This filter removes noise while keeping almost all reclaimable space visible. Use non-interactive mode to see everything.
</details>

<details>
<summary><b>Is it safe to delete these directories?</b></summary>

Most detected directories are safe to delete and can be regenerated:
- `node_modules` - Run `npm install`
- `.venv` - Recreate with `python -m venv .venv`
- `target` - Rebuild with `cargo build`
- Caches - Automatically regenerated

**However:** IDE settings (`.idea`, `.vscode`) contain your preferences. Review before deleting!
</details>

<details>
<summary><b>Can I add custom patterns?</b></summary>

Yes! Edit `src/utils.rs` and add your pattern to the `is_temp_directory()` function:

```rust
pub fn is_temp_directory(name: &str) -> bool {
    matches!(
        name,
        "node_modules" | "target" | "your_pattern" | // ...
    )
}
```

Then run tests and update documentation.
</details>

<details>
<summary><b>How do I see all directories, not just ≥1 MB?</b></summary>

Use non-interactive mode or export to CSV:

```bash
# Print all directories
disk-cleanup-tool --path ~/projects

# Export all to CSV
disk-cleanup-tool --path ~/projects --output-csv all.csv
```
</details>

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🌟 Show Your Support

If this tool saved you disk space, give it a ⭐ on GitHub!

---

**Made with ❤️ and Rust** | [Report Bug](../../issues) | [Request Feature](../../issues)
