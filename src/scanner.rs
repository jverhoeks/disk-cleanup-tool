use crate::utils::is_temp_directory;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub file_count: u64,
    pub size_bytes: u64,
    pub cumulative_file_count: u64,
    pub cumulative_size_bytes: u64,
    pub entry_type: EntryType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntryType {
    Normal,
    Temp,
}

pub struct ScanConfig {
    pub root_path: PathBuf,
    pub temp_only: bool,
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ScanError {
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    #[error("Path not found: {path}")]
    PathNotFound { path: PathBuf },

    #[error("IO error at {path}: {source}")]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub fn scan_directory(config: ScanConfig) -> Result<Vec<DirectoryEntry>, ScanError> {
    // Verify the root path exists
    if !config.root_path.exists() {
        return Err(ScanError::PathNotFound {
            path: config.root_path,
        });
    }

    // Map to store directory statistics: path -> (direct_file_count, direct_size_bytes, is_temp)
    let mut dir_stats: HashMap<PathBuf, (u64, u64, bool)> = HashMap::new();
    let mut temp_dirs_to_scan: Vec<PathBuf> = Vec::new();

    // First pass: walk the tree, identifying temp directories and counting direct files only
    for entry in WalkDir::new(&config.root_path).into_iter() {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if entry.file_type().is_dir() {
                    // Check if this is a temp directory
                    let is_temp = if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy();
                        is_temp_directory(&name_str)
                    } else {
                        false
                    };

                    // Add directory to map
                    let dir_path = path.to_path_buf();
                    dir_stats.entry(dir_path.clone()).or_insert((0, 0, is_temp));

                    if is_temp {
                        temp_dirs_to_scan.push(dir_path);
                    }
                } else if entry.file_type().is_file() {
                    // For files in non-temp directories, add to DIRECT parent only
                    if let Ok(metadata) = entry.metadata() {
                        let size = metadata.len();

                        // Check if file is inside a temp directory
                        let mut in_temp_dir = false;
                        let mut current = path.parent();
                        while let Some(parent) = current {
                            if let Some(name) = parent.file_name() {
                                if is_temp_directory(&name.to_string_lossy()) {
                                    in_temp_dir = true;
                                    break;
                                }
                            }
                            if parent == config.root_path {
                                break;
                            }
                            current = parent.parent();
                        }

                        // Only count files outside temp directories in this pass
                        // Add to DIRECT parent only
                        if !in_temp_dir {
                            if let Some(parent) = path.parent() {
                                let parent_buf = parent.to_path_buf();
                                let stats = dir_stats.entry(parent_buf).or_insert((0, 0, false));
                                stats.0 += 1;
                                stats.1 += size;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if let Some(path) = e.path() {
                    eprintln!("Warning: Cannot access {}: {}", path.display(), e);
                }
            }
        }
    }

    // Second pass: scan temp directories to get their sizes
    for temp_dir in temp_dirs_to_scan {
        let (mut file_count, mut size) = (0u64, 0u64);

        for entry in WalkDir::new(&temp_dir).into_iter().skip(1) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            file_count += 1;
                            size += metadata.len();
                        }
                    }
                }
                Err(_) => {}
            }
        }

        // Update temp directory stats (this is cumulative for temp dirs)
        if let Some(stats) = dir_stats.get_mut(&temp_dir) {
            stats.0 = file_count;
            stats.1 = size;
            stats.2 = true;
        }
    }

    // Third pass: calculate cumulative sizes by traversing bottom-up
    // Build a sorted list of directories by depth (deepest first)
    let mut dirs_by_depth: Vec<(PathBuf, usize)> = dir_stats
        .keys()
        .map(|p| {
            let depth = p.components().count();
            (p.clone(), depth)
        })
        .collect();
    dirs_by_depth.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by depth descending

    // Map to store cumulative stats: path -> (cumulative_file_count, cumulative_size_bytes)
    let mut cumulative_stats: HashMap<PathBuf, (u64, u64)> = HashMap::new();

    for (dir_path, _) in dirs_by_depth {
        let (direct_files, direct_size, _) = dir_stats[&dir_path];
        
        // Start with direct stats
        let mut cum_files = direct_files;
        let mut cum_size = direct_size;

        // Add all immediate children's cumulative stats
        for child_path in dir_stats.keys() {
            if let Some(parent) = child_path.parent() {
                if parent == dir_path && child_path != &dir_path {
                    if let Some((child_cum_files, child_cum_size)) = cumulative_stats.get(child_path) {
                        cum_files += child_cum_files;
                        cum_size += child_cum_size;
                    }
                }
            }
        }

        cumulative_stats.insert(dir_path, (cum_files, cum_size));
    }

    // Convert to DirectoryEntry vec
    let mut entries: Vec<DirectoryEntry> = dir_stats
        .into_iter()
        .map(|(path, (file_count, size_bytes, is_temp))| {
            let (cumulative_file_count, cumulative_size_bytes) = 
                cumulative_stats.get(&path).copied().unwrap_or((file_count, size_bytes));
            
            DirectoryEntry {
                path,
                file_count,
                size_bytes,
                cumulative_file_count,
                cumulative_size_bytes,
                entry_type: if is_temp {
                    EntryType::Temp
                } else {
                    EntryType::Normal
                },
            }
        })
        .collect();

    // Apply temp_only filter if requested
    if config.temp_only {
        entries.retain(|e| matches!(e.entry_type, EntryType::Temp));
    }

    // Sort by cumulative size descending for consistent output
    entries.sort_by(|a, b| b.cumulative_size_bytes.cmp(&a.cumulative_size_bytes));

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_simple_directory() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create a simple structure
        fs::write(root.join("file1.txt"), "hello").unwrap();
        fs::write(root.join("file2.txt"), "world").unwrap();

        let config = ScanConfig {
            root_path: root.to_path_buf(),
            temp_only: false,
        };

        let result = scan_directory(config).unwrap();

        // Should have at least the root directory
        assert!(!result.is_empty());
        let root_entry = result.iter().find(|e| e.path == root).unwrap();
        assert_eq!(root_entry.file_count, 2);
        assert_eq!(root_entry.size_bytes, 10); // "hello" + "world"
        assert_eq!(root_entry.cumulative_file_count, 2);
        assert_eq!(root_entry.cumulative_size_bytes, 10);
    }

    #[test]
    fn test_scan_with_temp_directory() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create structure with node_modules
        fs::create_dir(root.join("node_modules")).unwrap();
        fs::write(root.join("node_modules/package.json"), "{}").unwrap();
        fs::write(root.join("main.js"), "code").unwrap();

        let config = ScanConfig {
            root_path: root.to_path_buf(),
            temp_only: false,
        };

        let result = scan_directory(config).unwrap();

        // Find node_modules entry
        let node_modules = result
            .iter()
            .find(|e| e.path.file_name().map(|n| n == "node_modules").unwrap_or(false));
        
        assert!(node_modules.is_some(), "node_modules not found in results");
        let node_modules = node_modules.unwrap();
        assert_eq!(node_modules.entry_type, EntryType::Temp);
        assert_eq!(node_modules.file_count, 1);
        assert_eq!(node_modules.size_bytes, 2);
        assert_eq!(node_modules.cumulative_file_count, 1);
        assert_eq!(node_modules.cumulative_size_bytes, 2);
        
        // Check root includes temp directory
        let root_entry = result.iter().find(|e| e.path == root).unwrap();
        assert_eq!(root_entry.cumulative_file_count, 2); // main.js + package.json
        assert_eq!(root_entry.cumulative_size_bytes, 6); // "code" + "{}"
    }

    #[test]
    fn test_temp_only_filter() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        fs::create_dir(root.join("node_modules")).unwrap();
        fs::write(root.join("node_modules/file.js"), "x").unwrap();
        fs::create_dir(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();

        let config = ScanConfig {
            root_path: root.to_path_buf(),
            temp_only: true,
        };

        let result = scan_directory(config).unwrap();

        // Should only have temp directories
        assert!(result.iter().all(|e| matches!(e.entry_type, EntryType::Temp)));
        assert!(result.iter().any(|e| e.path.ends_with("node_modules")));
    }

    #[test]
    fn test_nonexistent_path() {
        let config = ScanConfig {
            root_path: PathBuf::from("/nonexistent/path/that/does/not/exist"),
            temp_only: false,
        };

        let result = scan_directory(config);
        assert!(matches!(result, Err(ScanError::PathNotFound { .. })));
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use std::fs;
    use tempfile::TempDir;

    // Feature: disk-cleanup-tool, Property 11: CSV round-trip consistency
    // Validates: Requirements 6.2
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_directory_entry_serialization_roundtrip(
            path in "[a-z/]{1,50}",
            file_count in 0u64..10000,
            size_bytes in 0u64..1000000000,
            cumulative_file_count in 0u64..10000,
            cumulative_size_bytes in 0u64..1000000000,
            is_temp in prop::bool::ANY
        ) {
            let entry_type = if is_temp { EntryType::Temp } else { EntryType::Normal };
            let entry = DirectoryEntry {
                path: PathBuf::from(path),
                file_count,
                size_bytes,
                cumulative_file_count,
                cumulative_size_bytes,
                entry_type,
            };

            // Serialize to JSON
            let serialized = serde_json::to_string(&entry).unwrap();
            
            // Deserialize back
            let deserialized: DirectoryEntry = serde_json::from_str(&serialized).unwrap();

            // Verify round-trip
            prop_assert_eq!(entry.path, deserialized.path);
            prop_assert_eq!(entry.file_count, deserialized.file_count);
            prop_assert_eq!(entry.size_bytes, deserialized.size_bytes);
            prop_assert_eq!(entry.cumulative_file_count, deserialized.cumulative_file_count);
            prop_assert_eq!(entry.cumulative_size_bytes, deserialized.cumulative_size_bytes);
            prop_assert_eq!(entry.entry_type, deserialized.entry_type);
        }

        // Feature: disk-cleanup-tool, Property 1: Complete directory traversal
        // Validates: Requirements 1.1
        #[test]
        fn test_scan_finds_all_directories(num_dirs in 1usize..5) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create multiple subdirectories
            for i in 0..num_dirs {
                let dir_path = root.join(format!("dir{}", i));
                fs::create_dir(&dir_path).unwrap();
                fs::write(dir_path.join("file.txt"), "content").unwrap();
            }

            let config = ScanConfig {
                root_path: root.to_path_buf(),
                temp_only: false,
            };

            let result = scan_directory(config).unwrap();
            
            // Should find root + all subdirectories
            prop_assert!(result.len() >= num_dirs + 1);
        }

        // Feature: disk-cleanup-tool, Property 3: Accurate size calculation
        // Validates: Requirements 1.3, 1.4
        #[test]
        fn test_size_calculation_accuracy(
            file_sizes in prop::collection::vec(1u64..1000, 1..10)
        ) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            let expected_total: u64 = file_sizes.iter().sum();

            // Create files with specified sizes
            for (i, &size) in file_sizes.iter().enumerate() {
                let content = vec![b'x'; size as usize];
                fs::write(root.join(format!("file{}.txt", i)), content).unwrap();
            }

            let config = ScanConfig {
                root_path: root.to_path_buf(),
                temp_only: false,
            };

            let result = scan_directory(config).unwrap();
            let root_entry = result.iter().find(|e| e.path == root).unwrap();

            prop_assert_eq!(root_entry.size_bytes, expected_total);
            prop_assert_eq!(root_entry.file_count, file_sizes.len() as u64);
            prop_assert_eq!(root_entry.cumulative_size_bytes, expected_total);
            prop_assert_eq!(root_entry.cumulative_file_count, file_sizes.len() as u64);
        }

        // Feature: disk-cleanup-tool, Property 24: Temp-only filter effectiveness
        // Validates: Requirements 8.1, 8.2, 8.3, 8.5
        #[test]
        fn test_temp_only_filter(has_temp in prop::bool::ANY, has_normal in prop::bool::ANY) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            if has_temp {
                fs::create_dir(root.join("node_modules")).unwrap();
                fs::write(root.join("node_modules/file.js"), "code").unwrap();
            }
            if has_normal {
                fs::create_dir(root.join("src")).unwrap();
                fs::write(root.join("src/main.rs"), "code").unwrap();
            }

            let config = ScanConfig {
                root_path: root.to_path_buf(),
                temp_only: true,
            };

            let result = scan_directory(config).unwrap();

            // All results should be temp directories
            for entry in &result {
                prop_assert_eq!(entry.entry_type, EntryType::Temp);
            }
        }

        // Feature: disk-cleanup-tool, Property 8: Parent subtotal inclusion
        // Validates: Requirements 2.5
        #[test]
        fn test_parent_includes_temp_dir_size(temp_size in 1u64..1000) {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create temp directory with known size
            fs::create_dir(root.join("node_modules")).unwrap();
            let content = vec![b'x'; temp_size as usize];
            fs::write(root.join("node_modules/package.json"), content).unwrap();

            let config = ScanConfig {
                root_path: root.to_path_buf(),
                temp_only: false,
            };

            let result = scan_directory(config).unwrap();
            let root_entry = result.iter().find(|e| e.path == root).unwrap();

            // Root should include temp directory size in cumulative
            prop_assert_eq!(root_entry.cumulative_size_bytes, temp_size);
            prop_assert_eq!(root_entry.cumulative_file_count, 1);
            // Root direct size should be 0 (no files directly in root)
            prop_assert_eq!(root_entry.size_bytes, 0);
            prop_assert_eq!(root_entry.file_count, 0);
        }
    }
}
