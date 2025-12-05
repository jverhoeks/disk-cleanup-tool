# Temporary Directory Detection Reference

This document provides a complete reference of all temporary directory patterns detected by the Disk Cleanup Tool, organized by ecosystem and purpose.

## Detection Strategy

The tool uses **exact name matching** to identify temporary directories. This conservative approach prevents accidental deletion of important directories:

**How it works:**
- ✅ `node_modules` is detected
- ❌ `my_node_modules` is NOT detected
- ❌ `node_modules_backup` is NOT detected

This conservative approach prevents accidental deletion of important directories.

## Complete List (60+ directories)

### Node.js / JavaScript Ecosystem

| Directory | Purpose |
|-----------|---------|
| `node_modules` | NPM/Yarn/PNPM dependencies |
| `bower_components` | Bower dependencies (legacy) |
| `.npm` | NPM cache |
| `.yarn` | Yarn cache |
| `.pnpm-store` | PNPM store |
| `.turbo` | Turborepo cache |
| `.parcel-cache` | Parcel bundler cache |
| `.webpack` | Webpack cache |
| `.rollup.cache` | Rollup cache |
| `.vite` | Vite cache |
| `.next` | Next.js build output |
| `.nuxt` | Nuxt.js build output |
| `.output` | Nitro/Nuxt output |
| `.vercel` | Vercel deployment cache |
| `.netlify` | Netlify deployment cache |

### Python Ecosystem

| Directory | Purpose |
|-----------|---------|
| `.venv` | Virtual environment (common convention) |
| `venv` | Virtual environment (Python default) |
| `env` | Virtual environment (alternative name) |
| `.env` | Virtual environment (alternative name) |
| `__pycache__` | Python bytecode cache |
| `.pytest_cache` | Pytest cache |
| `.mypy_cache` | MyPy type checker cache |
| `.tox` | Tox testing environments |
| `.eggs` | Python eggs |
| `*.egg-info` | Python package metadata |
| `.ipynb_checkpoints` | Jupyter notebook checkpoints |

### Rust Ecosystem

| Directory | Purpose |
|-----------|---------|
| `target` | Cargo build output |
| `.fingerprint` | Cargo build fingerprints |
| `.cargo` | Cargo local cache |

### Build Outputs

| Directory | Purpose |
|-----------|---------|
| `dist` | Distribution/build output (common) |
| `build` | Build output (common) |
| `out` | Output directory (common) |
| `.build` | Hidden build directory |
| `_build` | Build directory (Sphinx, Elixir) |
| `.gradle` | Gradle build cache |
| `.mvn` | Maven wrapper |

### General Caches

| Directory | Purpose |
|-----------|---------|
| `.cache` | Generic cache directory |
| `cache` | Generic cache directory |
| `.tmp` | Temporary files |
| `tmp` | Temporary files |
| `temp` | Temporary files |
| `.temp` | Temporary files |
| `.sass-cache` | Sass compilation cache |
| `.docusaurus` | Docusaurus cache |

### Version Managers

| Directory | Purpose |
|-----------|---------|
| `.nvm` | Node Version Manager |
| `.rvm` | Ruby Version Manager |
| `.rbenv` | Ruby environment manager |
| `.pyenv` | Python environment manager |

### IDEs & Editors

| Directory | Purpose |
|-----------|---------|
| `.idea` | JetBrains IDEs (IntelliJ, PyCharm, etc.) |
| `.vscode` | Visual Studio Code settings |
| `.vs` | Visual Studio settings |
| `.eclipse` | Eclipse IDE settings |
| `.settings` | Generic IDE settings |

### Testing & Coverage

| Directory | Purpose |
|-----------|---------|
| `coverage` | Test coverage reports |
| `.coverage` | Coverage.py data |
| `.nyc_output` | NYC (Istanbul) coverage |
| `htmlcov` | HTML coverage reports |

### Operating System

| Directory | Purpose |
|-----------|---------|
| `.DS_Store` | macOS metadata (file, not directory) |
| `Thumbs.db` | Windows thumbnail cache (file, not directory) |
| `.Trash` | Trash/recycle bin |

## Usage Examples

### Find all temporary directories
```bash
disk-cleanup-tool --path ~/projects --temp-only
```

### Find specific types
```bash
# Find all node_modules
disk-cleanup-tool --path ~/projects --temp-only | grep node_modules

# Find all Python virtual environments
disk-cleanup-tool --path ~/projects --temp-only | grep -E "(venv|\.venv)"

# Find all Rust target directories
disk-cleanup-tool --path ~/projects --temp-only | grep target
```

### Interactive cleanup
```bash
# Review and delete temporary directories interactively
disk-cleanup-tool --path ~/projects --temp-only --interactive
```

## Adding New Directories

To add more temporary directories to the detection list:

1. Edit `src/utils.rs`
2. Add the directory name to the `matches!` macro in `is_temp_directory()`
3. Add test cases in `test_is_temp_directory()`
4. Update the property test list in `test_known_temp_dirs_always_detected()`
5. Run `cargo test` to verify
6. Update this documentation

## Statistics

- **Total detected patterns**: 60+
- **Ecosystems covered**: 10+ (Node.js, Python, Rust, Java, Ruby, etc.)
- **Categories**: Dependencies, Caches, Build outputs, IDEs, Version managers, Testing

## Safety Notes

⚠️ **Important**: Always review directories before deletion!

- Some directories like `.venv` or `node_modules` can be safely deleted and regenerated
- Others like `.idea` or `.vscode` contain IDE settings you may want to keep
- Use `--interactive` mode to review before deleting
- Consider using `--output-csv` to save a scan for later review

## Performance Impact

The detection is extremely fast:
- Simple string matching (no regex)
- O(1) lookup time
- No filesystem operations needed
- Compiled into efficient match statement
