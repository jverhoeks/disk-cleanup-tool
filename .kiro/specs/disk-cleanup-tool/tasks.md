# Implementation Plan

- [x] 1. Set up Rust project structure and dependencies
  - Initialize Cargo project with appropriate name and structure
  - Add all required dependencies to Cargo.toml (clap, walkdir, rayon, csv, serde, crossterm, ratatui, thiserror, proptest, tempfile)
  - Create module structure: cli.rs, scanner.rs, csv_handler.rs, interactive.rs, deletion.rs, utils.rs
  - Set up basic error types using thiserror
  - _Requirements: All_

- [ ] 2. Implement core data models and utilities
  - [x] 2.1 Create DirectoryEntry struct with path, file_count, size_bytes, and entry_type fields
    - Implement Serialize and Deserialize traits for CSV compatibility
    - Create EntryType enum with Normal and Temp variants
    - _Requirements: 1.3, 2.1, 3.3_

  - [x]* 2.2 Write property test for DirectoryEntry serialization
    - **Property 11: CSV round-trip consistency**
    - **Validates: Requirements 6.2**

  - [x] 2.3 Implement utility functions for temp directory detection and size formatting
    - Write is_temp_directory() function checking against known temp directory names
    - Write format_size() function converting bytes to human-readable format (KB, MB, GB, TB)
    - _Requirements: 2.1, 7.2_

  - [x]* 2.4 Write property test for temp directory detection
    - **Property 5: Temporary directory classification**
    - **Validates: Requirements 2.1**

  - [x]* 2.5 Write property test for size formatting
    - **Property 22: Human-readable size formatting**
    - **Validates: Requirements 7.2**

- [ ] 3. Implement directory scanner
  - [x] 3.1 Create ScanConfig struct and scan_directory function signature
    - Define ScanConfig with root_path and temp_only fields
    - Define ScanError enum with PermissionDenied, PathNotFound, and IoError variants
    - _Requirements: 1.1, 8.1_

  - [x] 3.2 Implement recursive directory traversal logic
    - Use walkdir crate to traverse directories
    - Implement logic to stop at temporary directory boundaries
    - Collect DirectoryEntry for each directory
    - _Requirements: 1.1, 2.2, 2.3_

  - [x] 3.3 Implement file counting and size calculation
    - Count files in each directory including subdirectories
    - Calculate total size by summing all file sizes
    - Ensure parent directories include temporary directory totals
    - _Requirements: 1.3, 1.4, 2.5_

  - [x]* 3.4 Write property test for complete traversal
    - **Property 1: Complete directory traversal**
    - **Validates: Requirements 1.1**

  - [x]* 3.5 Write property test for accurate counting
    - **Property 2: Accurate file counting**
    - **Property 3: Accurate size calculation**
    - **Validates: Requirements 1.3, 1.4**

  - [x] 3.6 Implement error handling for permission errors
    - Catch permission denied errors during traversal
    - Log errors and continue processing remaining directories
    - _Requirements: 1.5_

  - [x]* 3.7 Write property test for error resilience
    - **Property 4: Error resilience**
    - **Validates: Requirements 1.5**

  - [x] 3.8 Implement temp-only filtering
    - Filter results to include only temporary directories when temp_only is true
    - _Requirements: 8.1, 8.2_

  - [x]* 3.9 Write property test for temp-only filter
    - **Property 24: Temp-only filter effectiveness**
    - **Validates: Requirements 8.1, 8.2, 8.3, 8.5**

  - [x]* 3.10 Write property test for reporting boundaries
    - **Property 7: Reporting boundary correctness**
    - **Validates: Requirements 2.3, 2.4**

  - [x]* 3.11 Write property test for parent subtotals
    - **Property 8: Parent subtotal inclusion**
    - **Validates: Requirements 2.5**

- [x] 4. Checkpoint - Ensure scanner tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Implement CSV handler
  - [x] 5.1 Create write_csv function
    - Write CSV with header row: path, files, size_bytes, type
    - Format size values as integers
    - Mark temporary directories as "temp" and normal as "normal"
    - Handle file overwriting
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

  - [x]* 5.2 Write property test for CSV type labeling
    - **Property 9: CSV type labeling**
    - **Validates: Requirements 3.3**

  - [x]* 5.3 Write property test for CSV size formatting
    - **Property 10: CSV size formatting**
    - **Validates: Requirements 3.5**

  - [x] 5.4 Create read_csv function
    - Parse CSV file with path, files, size_bytes, type columns
    - Return Vec<DirectoryEntry>
    - Define CsvError enum with appropriate variants
    - _Requirements: 6.1, 6.2_

  - [x] 5.5 Implement CSV error handling
    - Detect missing required columns
    - Report line numbers for parse errors
    - Handle malformed CSV files
    - _Requirements: 6.4, 9.4_

  - [x]* 5.6 Write property test for CSV round-trip
    - **Property 11: CSV round-trip consistency**
    - **Validates: Requirements 6.2**

  - [x]* 5.7 Write property test for malformed CSV handling
    - **Property 20: Malformed CSV error handling**
    - **Validates: Requirements 6.4**

  - [x]* 5.8 Write property test for CSV error details
    - **Property 27: CSV error detail**
    - **Validates: Requirements 9.4**

- [ ] 6. Implement CLI argument parser
  - [x] 6.1 Create CliArgs struct with clap
    - Add path field (optional PathBuf)
    - Add output_csv field (optional PathBuf)
    - Add input_csv field (optional PathBuf)
    - Add temp_only flag (bool)
    - Add interactive flag (bool)
    - _Requirements: 1.1, 1.2, 6.1, 8.1_

  - [x] 6.2 Implement parse_args function
    - Use clap to parse command-line arguments
    - Set up help text and usage examples
    - _Requirements: 1.1, 1.2_

  - [x] 6.3 Write unit test for default path behavior
    - Verify that when no path is provided, current directory is used
    - _Requirements: 1.2_

- [ ] 7. Implement interactive UI
  - [x] 7.1 Create InteractiveSession struct
    - Store entries, selected indices, current_page, page_size
    - Define InteractiveError enum
    - _Requirements: 4.1, 4.3_

  - [x] 7.2 Implement entry display with sorting
    - Sort entries by size in descending order
    - Display top 25 entries with path, file count, and human-readable size
    - Show visual indicators for selected entries
    - Display keyboard command instructions
    - _Requirements: 4.1, 4.2, 7.1, 7.3_

  - [x]* 7.3 Write property test for sorting
    - **Property 12: Top N sorting**
    - **Validates: Requirements 4.1**

  - [x]* 7.4 Write property test for display completeness
    - **Property 13: Display completeness**
    - **Validates: Requirements 4.2**

  - [x]* 7.5 Write property test for selection indicators
    - **Property 21: Selection indicator visibility**
    - **Validates: Requirements 7.1**

  - [x] 7.6 Implement keyboard input handling
    - Handle spacebar for selection toggle
    - Handle Enter for pagination
    - Handle 'd' key for proceeding to deletion
    - Handle 'q' key for quitting
    - Handle arrow keys or j/k for navigation
    - _Requirements: 4.3, 4.4, 5.1_

  - [x]* 7.7 Write property test for selection toggle
    - **Property 14: Selection toggle**
    - **Validates: Requirements 4.3**

  - [x]* 7.8 Write property test for pagination
    - **Property 15: Pagination advancement**
    - **Validates: Requirements 4.4**

  - [x] 7.9 Implement end-of-list handling
    - Display message when no more directories remain
    - _Requirements: 4.5_

  - [x] 7.10 Implement run method returning selected paths
    - Return Vec<PathBuf> of selected directories
    - _Requirements: 5.1_

- [ ] 8. Implement deletion manager
  - [x] 8.1 Create confirm_deletion function
    - Display all selected directories with total combined size
    - Prompt user to type "yes" to confirm
    - Return true only if user types exactly "yes"
    - _Requirements: 5.2, 5.3_

  - [x]* 8.2 Write property test for confirmation display
    - **Property 16: Confirmation display completeness**
    - **Validates: Requirements 5.2**

  - [x] 8.3 Write unit test for confirmation prompt
    - Test that "yes" returns true
    - Test that other inputs return false
    - _Requirements: 5.3_

  - [x] 8.4 Create delete_directories function
    - Recursively delete each selected directory
    - Continue on errors, collecting failures
    - Return DeletionReport with successful, failed, and total_freed_bytes
    - Define DeletionError enum
    - _Requirements: 5.4, 9.2_

  - [x]* 8.5 Write property test for deletion execution
    - **Property 17: Deletion execution**
    - **Validates: Requirements 5.4**

  - [x]* 8.6 Write property test for deletion cancellation
    - **Property 18: Deletion cancellation**
    - **Validates: Requirements 5.5**

  - [x] 8.7 Implement deletion error handling
    - Handle permission errors gracefully
    - Display descriptive error messages with path and reason
    - _Requirements: 9.2, 9.5_

  - [x]* 8.8 Write property test for batch deletion resilience
    - **Property 26: Batch deletion resilience**
    - **Validates: Requirements 9.2**

  - [x]* 8.9 Write property test for permission error handling
    - **Property 28: Permission error handling**
    - **Validates: Requirements 9.5**

- [ ] 9. Checkpoint - Ensure all component tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. Implement main application flow
  - [x] 10.1 Create main function with CLI routing
    - Parse CLI arguments
    - Route to scan mode or interactive mode based on arguments
    - Handle path validation and non-existent path errors
    - _Requirements: 1.2, 6.3, 9.3_

  - [x] 10.2 Write unit test for non-existent path handling
    - Verify error message and non-zero exit code
    - _Requirements: 9.3_

  - [x] 10.3 Implement scan mode workflow
    - Call scan_directory with provided or default path
    - Write results to CSV if output path provided
    - Display summary statistics
    - Optionally proceed to interactive mode
    - _Requirements: 1.1, 1.2, 3.1, 6.5_

  - [x] 10.4 Write unit test for CSV loading workflow
    - Verify that loading CSV skips scanning
    - _Requirements: 6.3, 6.5_

  - [x] 10.5 Implement interactive mode workflow
    - Load entries from CSV or scan results
    - Apply temp-only filter if specified
    - Create and run InteractiveSession
    - Call deletion manager with selected paths
    - Display final results
    - _Requirements: 4.1, 5.1, 6.3, 8.3_

  - [x]* 10.6 Write property test for CSV loading
    - **Property 19: CSV loading**
    - **Validates: Requirements 6.1**

  - [x]* 10.7 Write property test for error message descriptiveness
    - **Property 25: Error message descriptiveness**
    - **Validates: Requirements 9.1**

  - [x]* 10.8 Write property test for operation result messaging
    - **Property 23: Operation result messaging**
    - **Validates: Requirements 7.5**

- [ ] 11. Add progress indicators and user feedback
  - [x] 11.1 Implement progress display for scanning
    - Show periodic updates during directory traversal
    - Display count of directories processed
    - _Requirements: 7.4_

  - [x] 11.2 Implement success and error messaging
    - Display appropriate messages after operations complete
    - Include relevant details (files deleted, space freed, errors encountered)
    - _Requirements: 7.5_

- [x] 12. Final checkpoint - Integration testing and polish
  - Ensure all tests pass, ask the user if questions arise.
  - Verify complete workflows: scan-to-CSV, CSV-to-interactive, interactive-to-deletion
  - Test with real directory structures
  - Verify error handling across all components
