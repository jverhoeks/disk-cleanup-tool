use crate::utils::format_size;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DeletionError {
    #[error("Permission denied for {path}")]
    PermissionDenied { path: PathBuf },

    #[error("Failed to delete {path}: {reason}")]
    DeletionFailed { path: PathBuf, reason: String },
}

pub struct DeletionReport {
    pub successful: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
    pub total_freed_bytes: u64,
}

pub fn confirm_deletion(paths: &[PathBuf]) -> bool {
    if paths.is_empty() {
        return false;
    }

    // Calculate total size
    let mut total_size = 0u64;
    for path in paths {
        if let Ok(size) = calculate_dir_size(path) {
            total_size += size;
        }
    }

    println!("\n=== DELETION CONFIRMATION ===");
    println!("You are about to delete {} directories:", paths.len());
    for path in paths {
        println!("  - {}", path.display());
    }
    println!("\nTotal size to be freed: {}", format_size(total_size));
    println!("\nThis action cannot be undone!");
    print!("Type 'yes' to confirm deletion: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim() == "yes"
}

pub fn delete_directories(paths: &[PathBuf]) -> Result<DeletionReport, DeletionError> {
    let mut report = DeletionReport {
        successful: Vec::new(),
        failed: Vec::new(),
        total_freed_bytes: 0,
    };

    for path in paths {
        // Calculate size before deletion
        let size = calculate_dir_size(path).unwrap_or(0);

        match fs::remove_dir_all(path) {
            Ok(_) => {
                report.successful.push(path.clone());
                report.total_freed_bytes += size;
                println!("✓ Deleted: {}", path.display());
            }
            Err(e) => {
                let reason = e.to_string();
                report.failed.push((path.clone(), reason.clone()));
                eprintln!("✗ Failed to delete {}: {}", path.display(), reason);
            }
        }
    }

    Ok(report)
}

fn calculate_dir_size(path: &PathBuf) -> io::Result<u64> {
    let mut total = 0u64;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total += metadata.len();
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_delete_directories() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test directories
        let dir1 = root.join("dir1");
        let dir2 = root.join("dir2");
        fs::create_dir(&dir1).unwrap();
        fs::create_dir(&dir2).unwrap();
        fs::write(dir1.join("file.txt"), "content").unwrap();
        fs::write(dir2.join("file.txt"), "content").unwrap();

        let paths = vec![dir1.clone(), dir2.clone()];

        let report = delete_directories(&paths).unwrap();

        assert_eq!(report.successful.len(), 2);
        assert_eq!(report.failed.len(), 0);
        assert!(report.total_freed_bytes > 0);
        assert!(!dir1.exists());
        assert!(!dir2.exists());
    }

    #[test]
    fn test_delete_nonexistent_directory() {
        let paths = vec![PathBuf::from("/nonexistent/path")];

        let report = delete_directories(&paths).unwrap();

        assert_eq!(report.successful.len(), 0);
        assert_eq!(report.failed.len(), 1);
    }

    #[test]
    fn test_calculate_dir_size() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        fs::write(root.join("file1.txt"), "hello").unwrap();
        fs::write(root.join("file2.txt"), "world").unwrap();

        let size = calculate_dir_size(&root.to_path_buf()).unwrap();
        assert_eq!(size, 10); // "hello" + "world"
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::TempDir;

    // Feature: disk-cleanup-tool, Property 17: Deletion execution
    // Validates: Requirements 5.4
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn test_delete_directories_removes_all(
            num_dirs in 1usize..5
        ) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            let mut paths = Vec::new();
            for i in 0..num_dirs {
                let dir_path = root.join(format!("dir{}", i));
                fs::create_dir(&dir_path).unwrap();
                fs::write(dir_path.join("file.txt"), "content").unwrap();
                paths.push(dir_path);
            }

            // All directories should exist
            for path in &paths {
                prop_assert!(path.exists());
            }

            let report = delete_directories(&paths).unwrap();

            // All should be deleted
            prop_assert_eq!(report.successful.len(), num_dirs);
            prop_assert_eq!(report.failed.len(), 0);

            // Verify they're gone
            for path in &paths {
                prop_assert!(!path.exists());
            }
        }

        // Feature: disk-cleanup-tool, Property 26: Batch deletion resilience
        // Validates: Requirements 9.2
        #[test]
        fn test_deletion_continues_on_error(
            num_good in 1usize..3
        ) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            let mut paths = Vec::new();
            
            // Create some valid directories
            for i in 0..num_good {
                let dir_path = root.join(format!("good{}", i));
                fs::create_dir(&dir_path).unwrap();
                paths.push(dir_path);
            }

            // Add a nonexistent path
            paths.push(PathBuf::from("/nonexistent/path"));

            let report = delete_directories(&paths).unwrap();

            // Should have some successes and some failures
            prop_assert!(report.successful.len() > 0);
            prop_assert!(report.failed.len() > 0);
            prop_assert_eq!(report.successful.len() + report.failed.len(), paths.len());
        }
    }
}
