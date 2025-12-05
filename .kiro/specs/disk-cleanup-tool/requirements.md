# Requirements Document

## Introduction

This document specifies requirements for a Rust-based disk cleanup tool that recursively analyzes directory structures, identifies temporary directories (like node_modules, .venv, dist), calculates storage usage, and provides an interactive interface for reviewing and deleting space-consuming directories. The tool helps developers reclaim disk space by identifying and removing build artifacts and dependency directories across multiple projects.

## Glossary

- **Disk Cleanup Tool**: The Rust application that analyzes and cleans disk space
- **Temporary Directory**: Directories that can be safely deleted and regenerated (e.g., node_modules, .venv, dist, target, build)
- **Normal Directory**: Any directory that is not classified as temporary
- **Subtotal**: Aggregated file count and size for a directory and all its subdirectories
- **CSV Report**: A comma-separated values file containing the analysis results
- **Interactive Selection Interface**: A terminal-based UI allowing users to browse and select directories for deletion

## Requirements

### Requirement 1

**User Story:** As a developer, I want to analyze a directory tree starting from a specified path, so that I can understand the storage usage across my projects.

#### Acceptance Criteria

1. WHEN the user provides a directory path as input, THE Disk Cleanup Tool SHALL recursively traverse all subdirectories starting from that path
2. WHEN no path is provided, THE Disk Cleanup Tool SHALL use the current working directory as the starting point
3. WHEN traversing directories, THE Disk Cleanup Tool SHALL calculate the total number of files and total size in bytes for each directory
4. WHEN calculating directory sizes, THE Disk Cleanup Tool SHALL include all files within subdirectories in the subtotal
5. WHEN the traversal encounters permission errors or inaccessible directories, THE Disk Cleanup Tool SHALL log the error and continue processing remaining directories

### Requirement 2

**User Story:** As a developer, I want the tool to identify temporary directories like node_modules and .venv, so that I can distinguish between essential code and regenerable artifacts.

#### Acceptance Criteria

1. WHEN the Disk Cleanup Tool encounters a directory named node_modules, .venv, dist, target, build, .next, .nuxt, __pycache__, or .cache, THE Disk Cleanup Tool SHALL classify it as a temporary directory
2. WHEN a temporary directory is encountered, THE Disk Cleanup Tool SHALL traverse and count its contents
3. WHEN a temporary directory is encountered, THE Disk Cleanup Tool SHALL report only the temporary directory itself with its total size and file count, not its subdirectories
4. WHEN a normal directory is encountered, THE Disk Cleanup Tool SHALL report it with its aggregated statistics including all subdirectories
5. WHEN calculating parent directory subtotals, THE Disk Cleanup Tool SHALL include the size and file count of temporary directories

### Requirement 3

**User Story:** As a developer, I want the analysis results saved to a CSV file, so that I can review the data in spreadsheet applications or process it programmatically.

#### Acceptance Criteria

1. WHEN the analysis completes, THE Disk Cleanup Tool SHALL generate a CSV file with columns for path, file count, size in bytes, and directory type
2. WHEN writing the CSV file, THE Disk Cleanup Tool SHALL include a header row with column names: path, files, size_bytes, type
3. WHEN writing directory entries, THE Disk Cleanup Tool SHALL mark temporary directories with type "temp" and normal directories with type "normal"
4. WHEN the CSV file already exists, THE Disk Cleanup Tool SHALL overwrite it with the new analysis results
5. WHEN writing size values, THE Disk Cleanup Tool SHALL format them as integers representing bytes

### Requirement 4

**User Story:** As a developer, I want to see the top 25 largest directories in an interactive interface, so that I can quickly identify the biggest space consumers.

#### Acceptance Criteria

1. WHEN the user requests the interactive interface, THE Disk Cleanup Tool SHALL display the top 25 directories sorted by size in descending order
2. WHEN displaying directories, THE Disk Cleanup Tool SHALL show the path, file count, and human-readable size for each entry
3. WHEN the user presses the spacebar on a directory entry, THE Disk Cleanup Tool SHALL toggle the selection state of that directory
4. WHEN the user presses Enter, THE Disk Cleanup Tool SHALL display the next 25 directories if more entries exist
5. WHEN no more directories remain, THE Disk Cleanup Tool SHALL display a message indicating the end of the list

### Requirement 5

**User Story:** As a developer, I want to select multiple directories and delete them after confirmation, so that I can safely reclaim disk space.

#### Acceptance Criteria

1. WHEN the user has selected one or more directories, THE Disk Cleanup Tool SHALL provide a command to proceed to confirmation
2. WHEN the user proceeds to confirmation, THE Disk Cleanup Tool SHALL display all selected directories with their total combined size
3. WHEN the confirmation prompt is displayed, THE Disk Cleanup Tool SHALL require the user to type "yes" to proceed with deletion
4. WHEN the user types "yes", THE Disk Cleanup Tool SHALL recursively delete all selected directories
5. WHEN the user types anything other than "yes", THE Disk Cleanup Tool SHALL cancel the deletion and return to the selection interface

### Requirement 6

**User Story:** As a developer, I want to load a previously generated CSV file and use it for interactive selection, so that I don't need to re-scan the filesystem.

#### Acceptance Criteria

1. WHEN the user provides a CSV file path as input, THE Disk Cleanup Tool SHALL load the analysis data from that file
2. WHEN loading a CSV file, THE Disk Cleanup Tool SHALL parse the path, file count, size, and type columns
3. WHEN the CSV file is successfully loaded, THE Disk Cleanup Tool SHALL proceed directly to the interactive selection interface
4. WHEN the CSV file is malformed or missing required columns, THE Disk Cleanup Tool SHALL display an error message and exit
5. WHEN loading from CSV, THE Disk Cleanup Tool SHALL skip the filesystem scanning phase

### Requirement 7

**User Story:** As a developer, I want clear visual feedback in the terminal interface, so that I can easily understand the current state and available actions.

#### Acceptance Criteria

1. WHEN displaying the interactive interface, THE Disk Cleanup Tool SHALL show visual indicators for selected directories
2. WHEN displaying directory sizes, THE Disk Cleanup Tool SHALL format them in human-readable units (KB, MB, GB, TB)
3. WHEN displaying the interface, THE Disk Cleanup Tool SHALL show instructions for available keyboard commands
4. WHEN performing operations, THE Disk Cleanup Tool SHALL display progress indicators for long-running tasks
5. WHEN operations complete, THE Disk Cleanup Tool SHALL display success or error messages with relevant details

### Requirement 8

**User Story:** As a developer, I want to filter results to show only temporary directories, so that I can focus exclusively on space that can be easily reclaimed.

#### Acceptance Criteria

1. WHEN the user provides a temp-only filter option, THE Disk Cleanup Tool SHALL include only temporary directories in the analysis results
2. WHEN the temp-only filter is active during scanning, THE Disk Cleanup Tool SHALL skip reporting normal directories in the CSV output
3. WHEN the temp-only filter is active in the interactive interface, THE Disk Cleanup Tool SHALL display only temporary directories in the top 25 list
4. WHEN the temp-only filter is not specified, THE Disk Cleanup Tool SHALL include both temporary and normal directories in results
5. WHEN loading from CSV with the temp-only filter, THE Disk Cleanup Tool SHALL filter the loaded data to show only entries with type "temp"

### Requirement 9

**User Story:** As a developer, I want the tool to handle errors gracefully, so that I can understand what went wrong and take corrective action.

#### Acceptance Criteria

1. WHEN a filesystem operation fails, THE Disk Cleanup Tool SHALL display a descriptive error message including the path and error reason
2. WHEN deletion fails for a selected directory, THE Disk Cleanup Tool SHALL continue attempting to delete remaining selected directories
3. WHEN the starting path does not exist, THE Disk Cleanup Tool SHALL display an error message and exit with a non-zero status code
4. WHEN CSV parsing fails, THE Disk Cleanup Tool SHALL display the line number and parsing error details
5. WHEN the user lacks permissions to delete a directory, THE Disk Cleanup Tool SHALL display a permission error and skip that directory
