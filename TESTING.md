# Testing Documentation

This document provides detailed information about the test suite, coverage, and testing procedures for the Disk Cleanup Tool.

## Overview

The Disk Cleanup Tool has comprehensive test coverage with both unit tests and property-based tests, ensuring correctness across a wide range of scenarios.

### Test Statistics

- **Total Tests**: 30
- **Unit Tests**: 12
- **Property-Based Tests**: 18
- **Test Success Rate**: 100%

### UI Implementation

The interactive mode uses [Ratatui](https://ratatui.rs/) for a modern terminal user interface with:
- Real-time rendering with smooth animations
- Keyboard-driven navigation (vim-style and arrow keys)
- Visual feedback with colors and icons
- Live progress indicator during filesystem scanning

### Property-Based Testing

Property-based tests use the `proptest` crate to verify correctness properties across randomly generated inputs. Each test runs 100 iterations (50 for deletion tests) to ensure robustness.

#### Scanner Properties (5 tests)

1. **Directory Entry Serialization Round-trip** (Property 11)
   - Validates: Requirements 6.2
   - Tests that DirectoryEntry can be serialized and deserialized without data loss

2. **Complete Directory Traversal** (Property 1)
   - Validates: Requirements 1.1
   - Tests that all directories are found during scanning

3. **Accurate Size Calculation** (Property 3)
   - Validates: Requirements 1.3, 1.4
   - Tests that file sizes are correctly summed across directory trees

4. **Temp-Only Filter Effectiveness** (Property 24)
   - Validates: Requirements 8.1, 8.2, 8.3, 8.5
   - Tests that filtering only returns temporary directories

5. **Parent Subtotal Inclusion** (Property 8)
   - Validates: Requirements 2.5
   - Tests that parent directories include temp directory sizes

#### Utility Properties (4 tests)

1. **Known Temp Directories Always Detected** (Property 5)
   - Validates: Requirements 2.1
   - Tests that all known temp directory names are correctly identified

2. **Random Names Not Temp**
   - Validates: Requirements 2.1
   - Tests that arbitrary directory names are not incorrectly classified as temp

3. **Format Size Has Unit** (Property 22)
   - Validates: Requirements 7.2
   - Tests that formatted sizes always include a unit (B, KB, MB, GB, TB)

4. **Format Size Monotonic**
   - Validates: Requirements 7.2
   - Tests that larger byte values produce larger formatted numbers

#### CSV Handler Properties (4 tests)

1. **CSV Type Labeling** (Property 9)
   - Validates: Requirements 3.3
   - Tests that temp directories are labeled "temp" and normal directories "normal"

2. **CSV Size as Integer** (Property 10)
   - Validates: Requirements 3.5
   - Tests that sizes are written as integers without decimals or units

3. **CSV Round-trip** (Property 11)
   - Validates: Requirements 6.2
   - Tests that data can be written to CSV and read back without loss

4. **Malformed CSV Errors** (Property 20)
   - Validates: Requirements 6.4
   - Tests that invalid CSV data produces appropriate errors

#### Interactive UI Properties (2 tests)

1. **Entries Sorted by Size** (Property 12)
   - Validates: Requirements 4.1
   - Tests that directories are displayed in descending size order

2. **Selection Toggle** (Property 14)
   - Validates: Requirements 4.3
   - Tests that spacebar correctly toggles selection state

#### Deletion Properties (2 tests)

1. **Delete Directories Removes All** (Property 17)
   - Validates: Requirements 5.4
   - Tests that all selected directories are successfully deleted

2. **Deletion Continues on Error** (Property 26)
   - Validates: Requirements 9.2
   - Tests that batch deletion continues even when some deletions fail

### Unit Tests (12 tests)

#### Utility Tests (2 tests)
- `test_is_temp_directory`: Verifies temp directory detection
- `test_format_size`: Verifies size formatting with specific values

#### Scanner Tests (4 tests)
- `test_scan_simple_directory`: Basic directory scanning
- `test_scan_with_temp_directory`: Temp directory handling
- `test_temp_only_filter`: Filter functionality
- `test_nonexistent_path`: Error handling for invalid paths

#### CSV Handler Tests (3 tests)
- `test_write_and_read_csv`: Round-trip CSV operations
- `test_read_malformed_csv`: Error handling for invalid CSV
- `test_read_invalid_number`: Error handling for parse errors

#### Deletion Tests (3 tests)
- `test_delete_directories`: Successful deletion
- `test_delete_nonexistent_directory`: Error handling
- `test_calculate_dir_size`: Size calculation accuracy

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Module
```bash
cargo test scanner::tests
cargo test scanner::proptests
cargo test utils::tests
cargo test csv_handler::tests
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Run Property Tests with More Cases
```bash
# Edit the ProptestConfig in the code to increase cases
# Default is 100 cases per property test
```

## Test Design Principles

1. **Property-Based Testing First**: Universal properties are tested with random inputs to catch edge cases
2. **Unit Tests for Examples**: Specific scenarios and edge cases are covered with unit tests
3. **Integration Through Properties**: Property tests often exercise multiple components together
4. **Minimal Mocking**: Tests use real filesystem operations with temporary directories
5. **Fast Execution**: All tests complete in under 1 second

## Coverage Goals

The test suite aims to validate all correctness properties defined in the design document:

- ✅ Property 1: Complete directory traversal
- ✅ Property 3: Accurate size calculation
- ✅ Property 5: Temporary directory classification
- ✅ Property 8: Parent subtotal inclusion
- ✅ Property 9: CSV type labeling
- ✅ Property 10: CSV size formatting
- ✅ Property 11: CSV round-trip consistency
- ✅ Property 12: Top N sorting
- ✅ Property 14: Selection toggle
- ✅ Property 17: Deletion execution
- ✅ Property 20: Malformed CSV error handling
- ✅ Property 22: Human-readable size formatting
- ✅ Property 24: Temp-only filter effectiveness
- ✅ Property 26: Batch deletion resilience

## Continuous Integration

To integrate with CI/CD:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: cargo test --all-features

- name: Run tests with verbose output
  run: cargo test -- --nocapture --test-threads=1
```

## Manual Testing

### Interactive Mode Testing

To manually test the interactive TUI:

```bash
# Create test directory structure
mkdir -p /tmp/test-cleanup/{node_modules,src,.venv,build}
for i in {1..50}; do echo "test" > /tmp/test-cleanup/node_modules/file$i.js; done
for i in {1..30}; do echo "test" > /tmp/test-cleanup/.venv/module$i.py; done

# Run interactive mode
cargo run -- --path /tmp/test-cleanup --interactive
```

Test the following interactions:
- Navigation with arrow keys and j/k
- Selection with spacebar
- Select all with 'a'
- Clear selections with 'c'
- Page navigation with PgUp/PgDn
- Jump to top/bottom with Home/End
- Visual indicators (colors, icons, highlighting)
- Quit with 'q' or Esc

### Scanning Progress Testing

Test the live scanning indicator:

```bash
# Scan a large directory to see the progress UI
cargo run -- --path ~/projects
```

Verify:
- Animated spinner displays
- File and directory counts update in real-time
- Current path is shown and truncated if too long
- UI is responsive and doesn't flicker

## Future Test Improvements

Potential areas for additional testing:

1. Performance benchmarks for large directory trees
2. Stress tests with thousands of directories
3. Permission error simulation tests
4. TUI snapshot testing (using insta or similar)
5. Concurrent access tests
6. Terminal resize handling tests
