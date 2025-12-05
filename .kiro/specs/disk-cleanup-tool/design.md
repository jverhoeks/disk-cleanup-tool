# Design Document

## Overview

The Disk Cleanup Tool is a command-line application written in Rust that helps developers identify and remove space-consuming temporary directories across their project trees. The tool operates in two modes: scan mode (analyzing the filesystem and generating a CSV report) and interactive mode (loading results and allowing selective deletion). The architecture emphasizes performance through parallel directory traversal, safety through explicit confirmation, and usability through a terminal-based UI.

## Architecture

The application follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                     CLI Interface                        │
│              (Argument parsing & routing)                │
└────────────────┬────────────────────────────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
┌───────▼──────┐   ┌──────▼────────┐
│ Scan Mode    │   │ Interactive   │
│              │   │ Mode          │
└───┬──────────┘   └──────┬────────┘
    │                     │
    │              ┌──────▼────────┐
    │              │ CSV Loader    │
    │              └──────┬────────┘
    │                     │
┌───▼─────────────────────▼────────┐
│      Directory Scanner           │
│  (Recursive traversal & stats)   │
└───────────────┬──────────────────┘
                │
        ┌───────┴────────┐
        │                │
┌───────▼──────┐  ┌──────▼────────┐
│ CSV Writer   │  │ TUI Component │
│              │  │ (Selection &  │
│              │  │  Deletion)    │
└──────────────┘  └───────────────┘
```

### Key Design Decisions

1. **Parallel Traversal**: Use Rust's `rayon` crate for parallel directory walking to maximize performance on large directory trees
2. **Terminal UI**: Use `crossterm` for cross-platform terminal manipulation and `tui-rs` or similar for the interactive interface
3. **CSV Format**: Standard CSV with headers for easy integration with spreadsheet tools and scripting
4. **Atomic Operations**: Ensure CSV writes are atomic to prevent corruption
5. **Lazy Evaluation**: Only traverse temporary directories when needed, stopping at the boundary

## Components and Interfaces

### 1. CLI Parser (`cli.rs`)

Handles command-line argument parsing using the `clap` crate.

```rust
pub struct CliArgs {
    pub path: Option<PathBuf>,
    pub output_csv: Option<PathBuf>,
    pub input_csv: Option<PathBuf>,
    pub temp_only: bool,
    pub interactive: bool,
}

pub fn parse_args() -> CliArgs;
```

### 2. Directory Scanner (`scanner.rs`)

Core component responsible for filesystem traversal and statistics collection.

```rust
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub file_count: u64,
    pub size_bytes: u64,
    pub entry_type: EntryType,
}

pub enum EntryType {
    Normal,
    Temp,
}

pub struct ScanConfig {
    pub root_path: PathBuf,
    pub temp_only: bool,
}

pub fn scan_directory(config: ScanConfig) -> Result<Vec<DirectoryEntry>, ScanError>;
```

**Temporary Directory Detection**: Maintains a static list of known temporary directory names:
- `node_modules` (Node.js)
- `.venv`, `venv`, `__pycache__` (Python)
- `dist`, `build`, `.next`, `.nuxt` (Build outputs)
- `target` (Rust)
- `.cache`

### 3. CSV Handler (`csv_handler.rs`)

Manages reading and writing CSV files.

```rust
pub fn write_csv(entries: &[DirectoryEntry], path: &Path) -> Result<(), CsvError>;
pub fn read_csv(path: &Path) -> Result<Vec<DirectoryEntry>, CsvError>;
```

CSV Format:
```
path,files,size_bytes,type
/home/user/projects/org1/project1,150,2048576,normal
/home/user/projects/org1/project1/node_modules,5420,524288000,temp
```

### 4. Interactive UI (`interactive.rs`)

Terminal-based interface for browsing and selecting directories.

```rust
pub struct InteractiveSession {
    entries: Vec<DirectoryEntry>,
    selected: HashSet<usize>,
    current_page: usize,
    page_size: usize,
}

impl InteractiveSession {
    pub fn new(entries: Vec<DirectoryEntry>) -> Self;
    pub fn run(&mut self) -> Result<Vec<PathBuf>, InteractiveError>;
}
```

**Key Interactions**:
- `↑/↓` or `j/k`: Navigate entries
- `Space`: Toggle selection
- `Enter`: Next page / Confirm
- `d`: Proceed to deletion confirmation
- `q`: Quit without deleting

### 5. Deletion Manager (`deletion.rs`)

Handles safe deletion with confirmation.

```rust
pub fn confirm_deletion(paths: &[PathBuf]) -> bool;
pub fn delete_directories(paths: &[PathBuf]) -> Result<DeletionReport, DeletionError>;

pub struct DeletionReport {
    pub successful: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
    pub total_freed_bytes: u64,
}
```

### 6. Utilities (`utils.rs`)

Helper functions for formatting and common operations.

```rust
pub fn format_size(bytes: u64) -> String;  // e.g., "1.5 GB"
pub fn is_temp_directory(name: &str) -> bool;
```

## Data Models

### DirectoryEntry

The primary data structure representing a scanned directory:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    /// Full path to the directory
    pub path: PathBuf,
    
    /// Total number of files (including subdirectories)
    pub file_count: u64,
    
    /// Total size in bytes (including subdirectories)
    pub size_bytes: u64,
    
    /// Classification as normal or temporary
    pub entry_type: EntryType,
}
```

### ScanResult

Aggregated results from a directory scan:

```rust
pub struct ScanResult {
    pub entries: Vec<DirectoryEntry>,
    pub total_size: u64,
    pub total_files: u64,
    pub scan_duration: Duration,
    pub errors: Vec<ScanError>,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Property 1: Complete directory traversal
*For any* valid directory path, scanning should visit all accessible subdirectories and include them in the results.
**Validates: Requirements 1.1**

### Property 2: Accurate file counting
*For any* directory structure, the reported file count for each directory should equal the actual number of files in that directory and all its subdirectories.
**Validates: Requirements 1.3**

### Property 3: Accurate size calculation
*For any* directory structure, the reported size for each directory should equal the sum of all file sizes in that directory and all its subdirectories.
**Validates: Requirements 1.3, 1.4**

### Property 4: Error resilience
*For any* directory scan that encounters permission errors, the scan should continue processing remaining accessible directories and log all errors encountered.
**Validates: Requirements 1.5**

### Property 5: Temporary directory classification
*For any* directory with a name in the set {node_modules, .venv, venv, __pycache__, dist, build, .next, .nuxt, target, .cache}, it should be classified as type "temp".
**Validates: Requirements 2.1**

### Property 6: Temporary directory content analysis
*For any* temporary directory, the tool should traverse and count its contents to produce accurate file count and size statistics.
**Validates: Requirements 2.2**

### Property 7: Reporting boundary correctness
*For any* temporary directory with subdirectories, only the temporary directory itself should appear in results, not its subdirectories. For any normal directory with subdirectories, both the directory and its subdirectories should appear in results.
**Validates: Requirements 2.3, 2.4**

### Property 8: Parent subtotal inclusion
*For any* directory containing a temporary subdirectory, the parent directory's size and file count should include the temporary subdirectory's totals.
**Validates: Requirements 2.5**

### Property 9: CSV type labeling
*For any* directory entry written to CSV, temporary directories should have type "temp" and normal directories should have type "normal".
**Validates: Requirements 3.3**

### Property 10: CSV size formatting
*For any* size value written to CSV, it should be formatted as an integer without decimal points or unit suffixes.
**Validates: Requirements 3.5**

### Property 11: CSV round-trip consistency
*For any* set of directory entries, writing them to CSV and then reading the CSV back should produce equivalent directory entries with the same paths, file counts, sizes, and types.
**Validates: Requirements 6.2**

### Property 12: Top N sorting
*For any* list of directory entries displayed in interactive mode, the entries should be sorted by size in descending order.
**Validates: Requirements 4.1**

### Property 13: Display completeness
*For any* directory entry displayed in the interactive interface, the rendered output should contain the path, file count, and human-readable size.
**Validates: Requirements 4.2**

### Property 14: Selection toggle
*For any* directory entry in the interactive interface, pressing spacebar should toggle its selection state from unselected to selected or from selected to unselected.
**Validates: Requirements 4.3**

### Property 15: Pagination advancement
*For any* interactive session with more than 25 entries, pressing Enter should advance to display the next 25 entries.
**Validates: Requirements 4.4**

### Property 16: Confirmation display completeness
*For any* set of selected directories, the confirmation prompt should display all selected paths and the sum of their sizes.
**Validates: Requirements 5.2**

### Property 17: Deletion execution
*For any* set of selected directories, when the user confirms with "yes", all selected directories should be recursively deleted from the filesystem.
**Validates: Requirements 5.4**

### Property 18: Deletion cancellation
*For any* confirmation prompt, when the user types anything other than "yes", no directories should be deleted and the interface should return to selection mode.
**Validates: Requirements 5.5**

### Property 19: CSV loading
*For any* valid CSV file with the correct format, loading it should produce a list of directory entries that can be used in interactive mode.
**Validates: Requirements 6.1**

### Property 20: Malformed CSV error handling
*For any* CSV file that is malformed or missing required columns, attempting to load it should produce an error message and exit without proceeding to interactive mode.
**Validates: Requirements 6.4**

### Property 21: Selection indicator visibility
*For any* selected directory in the interactive interface, the rendered output should include a visual indicator distinguishing it from unselected directories.
**Validates: Requirements 7.1**

### Property 22: Human-readable size formatting
*For any* byte value displayed in the interface, it should be formatted with appropriate units (KB, MB, GB, TB) for readability.
**Validates: Requirements 7.2**

### Property 23: Operation result messaging
*For any* completed operation (scan, load, delete), the tool should display a message indicating success or failure with relevant details.
**Validates: Requirements 7.5**

### Property 24: Temp-only filter effectiveness
*For any* scan or load operation with the temp-only filter enabled, all results should have type "temp" and no normal directories should be included.
**Validates: Requirements 8.1, 8.2, 8.3, 8.5**

### Property 25: Error message descriptiveness
*For any* filesystem operation that fails, the error message should include both the path that caused the error and the reason for the failure.
**Validates: Requirements 9.1**

### Property 26: Batch deletion resilience
*For any* batch deletion operation where one or more deletions fail, the tool should continue attempting to delete remaining directories and report which succeeded and which failed.
**Validates: Requirements 9.2**

### Property 27: CSV error detail
*For any* CSV parsing error, the error message should include the line number where the error occurred and details about what went wrong.
**Validates: Requirements 9.4**

### Property 28: Permission error handling
*For any* directory that cannot be deleted due to insufficient permissions, the tool should display a permission error and continue processing remaining directories.
**Validates: Requirements 9.5**

## Error Handling

The application uses Rust's `Result` type throughout for explicit error handling. Custom error types are defined for each major component:

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
    
    #[error("Path not found: {path}")]
    PathNotFound { path: PathBuf },
    
    #[error("IO error at {path}: {source}")]
    IoError { path: PathBuf, source: std::io::Error },
}

#[derive(Debug, thiserror::Error)]
pub enum CsvError {
    #[error("Failed to write CSV: {0}")]
    WriteError(#[from] csv::Error),
    
    #[error("Failed to read CSV: {0}")]
    ReadError(#[from] csv::Error),
    
    #[error("Missing required column: {0}")]
    MissingColumn(String),
    
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DeletionError {
    #[error("Permission denied for {path}")]
    PermissionDenied { path: PathBuf },
    
    #[error("Failed to delete {path}: {reason}")]
    DeletionFailed { path: PathBuf, reason: String },
}
```

### Error Recovery Strategy

1. **Scan Errors**: Log and continue - one inaccessible directory shouldn't stop the entire scan
2. **CSV Errors**: Fail fast - corrupted data should not proceed to interactive mode
3. **Deletion Errors**: Continue with remaining deletions - report all failures at the end
4. **User Input Errors**: Prompt again - give users another chance to provide valid input

## Testing Strategy

The Disk Cleanup Tool will employ a comprehensive testing strategy combining unit tests for specific scenarios and property-based tests for universal correctness guarantees.

### Property-Based Testing

We will use the `proptest` crate for property-based testing in Rust. Each correctness property defined above will be implemented as a property-based test that verifies the behavior across randomly generated inputs.

**Configuration**:
- Each property test will run a minimum of 100 iterations
- Tests will use custom generators for directory structures, file sizes, and paths
- Each test will be tagged with a comment referencing its corresponding correctness property

**Tag Format**: `// Feature: disk-cleanup-tool, Property N: <property description>`

**Example Property Test Structure**:
```rust
#[test]
fn test_property_3_accurate_size_calculation() {
    // Feature: disk-cleanup-tool, Property 3: Accurate size calculation
    proptest!(|(dir_structure in arbitrary_directory_structure())| {
        let expected_size = calculate_expected_size(&dir_structure);
        let result = scan_directory(&dir_structure.root);
        prop_assert_eq!(result.size_bytes, expected_size);
    });
}
```

### Unit Testing

Unit tests will cover:
- Specific examples of temporary directory detection (node_modules, .venv, etc.)
- Edge cases like empty directories, single-file directories
- CSV header format validation
- Default path behavior (using current directory)
- Confirmation prompt with "yes" input
- Non-existent path error handling
- End-of-list pagination behavior

### Integration Testing

Integration tests will verify:
- Complete scan-to-CSV workflow
- CSV-to-interactive workflow
- Interactive selection and deletion workflow
- Error handling across component boundaries

### Test Utilities

Custom generators for property tests:
- `arbitrary_directory_structure()`: Generates random nested directory trees
- `arbitrary_file_sizes()`: Generates realistic file size distributions
- `arbitrary_temp_dir_names()`: Generates valid and invalid temp directory names
- `arbitrary_csv_content()`: Generates valid and malformed CSV data

## Performance Considerations

1. **Parallel Traversal**: Use `rayon` to parallelize directory walking across multiple cores
2. **Lazy Evaluation**: Stop traversing inside temporary directories once identified
3. **Buffered I/O**: Use buffered writers for CSV generation
4. **Memory Management**: Stream large directory lists rather than loading everything into memory
5. **Progress Feedback**: Update UI periodically during long scans to maintain responsiveness

## Security Considerations

1. **Path Validation**: Sanitize and validate all user-provided paths to prevent directory traversal attacks
2. **Confirmation Required**: Always require explicit "yes" confirmation before deletion
3. **Permission Checks**: Verify permissions before attempting deletion
4. **Atomic Operations**: Ensure CSV writes are atomic to prevent partial writes
5. **Error Disclosure**: Avoid leaking sensitive path information in error messages shown to users

## Dependencies

Key Rust crates:
- `clap` (v4): Command-line argument parsing
- `walkdir` or `ignore`: Directory traversal
- `rayon`: Parallel processing
- `csv`: CSV reading and writing
- `serde`: Serialization/deserialization
- `crossterm`: Terminal manipulation
- `tui` or `ratatui`: Terminal UI framework
- `thiserror`: Error type derivation
- `proptest`: Property-based testing
- `tempfile`: Temporary directories for testing

## Future Enhancements

Potential improvements for future versions:
1. Configuration file for custom temporary directory patterns
2. Dry-run mode showing what would be deleted without actually deleting
3. Export to other formats (JSON, SQLite)
4. Filtering by age (e.g., only show node_modules older than 30 days)
5. Parallel deletion for faster cleanup
6. Undo functionality with trash/recycle bin integration
7. Watch mode for continuous monitoring
