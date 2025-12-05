# Contributing to Disk Cleanup Tool

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/disk-cleanup-tool.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test scanner::tests
```

### Running Locally

```bash
cargo run -- --path test_dir --interactive
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for lints: `cargo clippy`
- Write tests for new features
- Update documentation as needed

## Areas for Contribution

### High Priority
- Additional temporary directory patterns (with justification)
- Performance improvements for large directory trees
- Bug fixes and error handling improvements

### Medium Priority
- Search/filter functionality in interactive mode
- Configurable size threshold (`--min-size` flag)
- Additional export formats (JSON, YAML)
- Directory tree view in TUI

### Low Priority
- Undo/redo for selections
- Batch operations (select by pattern)
- Detailed info panel for selected items
- Sort options (by name, type, etc.)

## Adding New Temporary Directory Patterns

When adding new patterns to detect:

1. **Verify the pattern is widely used** - Should be common across many projects
2. **Add to `src/utils.rs`** - Update the `is_temp_directory()` function
3. **Add tests** - Update both unit tests and property tests
4. **Update documentation** - Add to README.md "What Gets Detected?" section
5. **Provide rationale** - Explain why this pattern should be detected in your PR

Example:
```rust
// In src/utils.rs
pub fn is_temp_directory(name: &str) -> bool {
    matches!(
        name,
        "node_modules" | "target" | "your_new_pattern" | // ...
    )
}

// In tests
#[test]
fn test_is_temp_directory() {
    assert!(is_temp_directory("your_new_pattern"));
}
```

## Testing Guidelines

- Write unit tests for specific scenarios
- Use property-based tests for universal properties
- Test error cases and edge conditions
- Ensure tests are fast (< 1 second total)
- Use temporary directories for filesystem tests

## Documentation

When making changes:

- Update README.md if user-facing features change (all docs are consolidated there)
- Update CHANGELOG.md with your changes
- Add inline code comments for complex logic
- Update FAQ section in README if adding commonly asked features

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add entry to CHANGELOG.md under [Unreleased]
4. Provide clear description of changes
5. Reference any related issues

## Code Review

All submissions require review. We'll look for:

- Code quality and style
- Test coverage
- Documentation updates
- Performance implications
- Security considerations

## Questions?

Feel free to open an issue for:

- Bug reports
- Feature requests
- Questions about the codebase
- Clarification on contribution guidelines

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
