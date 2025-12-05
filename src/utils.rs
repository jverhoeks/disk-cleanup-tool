/// Check if a directory name indicates a temporary directory
pub fn is_temp_directory(name: &str) -> bool {
    matches!(
        name,
        // Node.js / JavaScript
        "node_modules"
            | ".npm"
            | ".yarn"
            | ".pnpm-store"
            | ".turbo"
            | ".parcel-cache"
            | ".webpack"
            | ".rollup.cache"
            | ".vite"
            | ".next"
            | ".nuxt"
            | ".output"
            | ".vercel"
            | ".netlify"
            | "bower_components"
            // Python
            | ".venv"
            | "venv"
            | "env"
            | ".env"
            | "__pycache__"
            | ".pytest_cache"
            | ".mypy_cache"
            | ".tox"
            | ".eggs"
            | "*.egg-info"
            | ".ipynb_checkpoints"
            // Rust
            | "target"
            | ".fingerprint"
            | ".cargo"
            // Build outputs
            | "dist"
            | "build"
            | "out"
            | ".build"
            | "_build"
            | ".gradle"
            | ".mvn"
            // Caches
            | ".cache"
            | "cache"
            | ".tmp"
            | "tmp"
            | "temp"
            | ".temp"
            // Version managers
            | ".nvm"
            | ".rvm"
            | ".rbenv"
            | ".pyenv"
            // IDEs and editors
            | ".idea"
            | ".vscode"
            | ".vs"
            | ".eclipse"
            | ".settings"
            // OS
            | ".DS_Store"
            | "Thumbs.db"
            | ".Trash"
            // Other
            | "coverage"
            | ".coverage"
            | ".nyc_output"
            | "htmlcov"
            | ".sass-cache"
            | ".docusaurus"
    )
}

/// Format bytes into human-readable size (KB, MB, GB, TB)
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_temp_directory() {
        // Test Node.js / JavaScript temp directories
        assert!(is_temp_directory("node_modules"));
        assert!(is_temp_directory(".npm"));
        assert!(is_temp_directory(".yarn"));
        assert!(is_temp_directory(".next"));
        assert!(is_temp_directory(".nuxt"));
        assert!(is_temp_directory(".turbo"));
        assert!(is_temp_directory(".vite"));
        
        // Test Python temp directories
        assert!(is_temp_directory(".venv"));
        assert!(is_temp_directory("venv"));
        assert!(is_temp_directory("__pycache__"));
        assert!(is_temp_directory(".pytest_cache"));
        assert!(is_temp_directory(".mypy_cache"));
        
        // Test Rust temp directories
        assert!(is_temp_directory("target"));
        assert!(is_temp_directory(".fingerprint"));
        assert!(is_temp_directory(".cargo"));
        
        // Test build outputs
        assert!(is_temp_directory("dist"));
        assert!(is_temp_directory("build"));
        assert!(is_temp_directory("out"));
        
        // Test caches
        assert!(is_temp_directory(".cache"));
        assert!(is_temp_directory("cache"));
        assert!(is_temp_directory("tmp"));
        
        // Test version managers
        assert!(is_temp_directory(".nvm"));
        assert!(is_temp_directory(".rvm"));
        assert!(is_temp_directory(".pyenv"));
        
        // Test IDEs
        assert!(is_temp_directory(".idea"));
        assert!(is_temp_directory(".vscode"));
        
        // Test coverage
        assert!(is_temp_directory("coverage"));
        assert!(is_temp_directory(".nyc_output"));

        // Test normal directories
        assert!(!is_temp_directory("src"));
        assert!(!is_temp_directory("lib"));
        assert!(!is_temp_directory("tests"));
        assert!(!is_temp_directory("my_project"));
        assert!(!is_temp_directory("public"));
        assert!(!is_temp_directory("assets"));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
        assert_eq!(format_size(1099511627776), "1.00 TB");
        assert_eq!(format_size(5368709120), "5.00 GB");
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // Feature: disk-cleanup-tool, Property 5: Temporary directory classification
    // Validates: Requirements 2.1
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_known_temp_dirs_always_detected(
            prefix in "[a-z]{0,10}",
            suffix in "[a-z]{0,10}"
        ) {
            // Known temp directory names should always be detected
            let temp_names = vec![
                "node_modules", ".venv", "venv", "__pycache__",
                "dist", "build", ".next", ".nuxt", "target", ".cache",
                ".fingerprint", ".nvm", ".npm", ".yarn", ".turbo",
                ".pytest_cache", ".mypy_cache", ".idea", ".vscode",
                "coverage", "tmp", "out"
            ];
            
            for name in temp_names {
                prop_assert!(is_temp_directory(name));
                
                // With prefix/suffix should NOT be detected (exact match only)
                if !prefix.is_empty() {
                    let with_prefix = format!("{}{}", prefix, name);
                    prop_assert!(!is_temp_directory(&with_prefix));
                }
                if !suffix.is_empty() {
                    let with_suffix = format!("{}{}", name, suffix);
                    prop_assert!(!is_temp_directory(&with_suffix));
                }
            }
        }

        #[test]
        fn test_random_names_not_temp(
            name in "[a-z_]{1,20}"
        ) {
            // Filter out actual temp directory names
            let temp_names = vec![
                "node_modules", "venv", "__pycache__",
                "dist", "build", "target"
            ];
            
            if !temp_names.contains(&name.as_str()) && !name.starts_with('.') {
                prop_assert!(!is_temp_directory(&name));
            }
        }

        // Feature: disk-cleanup-tool, Property 22: Human-readable size formatting
        // Validates: Requirements 7.2
        #[test]
        fn test_format_size_has_unit(bytes in 0u64..10000000000u64) {
            let formatted = format_size(bytes);
            
            // Should always contain a unit
            prop_assert!(
                formatted.contains(" B") ||
                formatted.contains(" KB") ||
                formatted.contains(" MB") ||
                formatted.contains(" GB") ||
                formatted.contains(" TB")
            );
        }

        #[test]
        fn test_format_size_monotonic(bytes1 in 0u64..1000000u64, bytes2 in 0u64..1000000u64) {
            // Larger byte values should have larger or equal numeric part
            // (when comparing same units)
            let formatted1 = format_size(bytes1);
            let formatted2 = format_size(bytes2);
            
            // Extract unit
            let unit1 = formatted1.split_whitespace().last().unwrap();
            let unit2 = formatted2.split_whitespace().last().unwrap();
            
            // Only compare if same unit
            if unit1 == unit2 {
                let num1: f64 = formatted1.split_whitespace().next().unwrap().parse().unwrap();
                let num2: f64 = formatted2.split_whitespace().next().unwrap().parse().unwrap();
                
                if bytes1 > bytes2 {
                    prop_assert!(num1 >= num2);
                } else if bytes1 < bytes2 {
                    prop_assert!(num1 <= num2);
                }
            }
        }
    }
}
