mod cli;
mod csv_handler;
mod deletion;
mod interactive;
mod scan_ui;
mod scanner;
mod utils;

use scanner::ScanConfig;
use std::env;
use std::process;

fn main() {
    let args = cli::parse_args();

    // Determine the starting path
    let root_path = args.path.unwrap_or_else(|| {
        env::current_dir().unwrap_or_else(|e| {
            eprintln!("Error: Cannot determine current directory: {}", e);
            process::exit(1);
        })
    });

    // Verify path exists
    if !root_path.exists() {
        eprintln!("Error: Path does not exist: {}", root_path.display());
        process::exit(1);
    }

    // Load entries from CSV or scan filesystem
    let entries = if let Some(input_csv) = args.input_csv {
        // Load from CSV
        match csv_handler::read_csv(&input_csv) {
            Ok(mut entries) => {
                println!("Loaded {} entries from {}", entries.len(), input_csv.display());
                
                // Apply temp_only filter if specified
                if args.temp_only {
                    entries.retain(|e| matches!(e.entry_type, scanner::EntryType::Temp));
                    println!("Filtered to {} temporary directories", entries.len());
                }
                
                entries
            }
            Err(e) => {
                eprintln!("Error reading CSV: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Scan filesystem with progress UI
        let config = ScanConfig {
            root_path: root_path.clone(),
            temp_only: args.temp_only,
        };

        match scan_ui::scan_with_progress(config) {
            Ok(entries) => {
                println!("✓ Scan complete! Found {} directories", entries.len());
                entries
            }
            Err(e) => {
                eprintln!("Error scanning directory: {}", e);
                process::exit(1);
            }
        }
    };

    // Write to CSV if output path specified
    if let Some(output_csv) = args.output_csv {
        match csv_handler::write_csv(&entries, &output_csv) {
            Ok(_) => println!("Results saved to {}", output_csv.display()),
            Err(e) => {
                eprintln!("Error writing CSV: {}", e);
                process::exit(1);
            }
        }
    }

    // Display summary
    if !entries.is_empty() {
        // Find the root entry (should be first after sorting by cumulative size)
        let root_entry = entries.iter().find(|e| e.path == root_path);
        
        if let Some(root) = root_entry {
            println!("\nSummary:");
            println!("  Total directories: {}", entries.len());
            println!("  Total files: {}", root.cumulative_file_count);
            println!("  Total size: {}", utils::format_size(root.cumulative_size_bytes));
        } else {
            println!("\nSummary:");
            println!("  Total directories: {}", entries.len());
        }
        
        // Show top 10
        println!("\nTop 10 largest directories:");
        for (i, entry) in entries.iter().take(10).enumerate() {
            println!(
                "  {}. {} - {} ({} files)",
                i + 1,
                entry.path.display(),
                utils::format_size(entry.cumulative_size_bytes),
                entry.cumulative_file_count
            );
        }
    }

    // Launch interactive mode if requested
    if args.interactive {
        if entries.is_empty() {
            println!("\nNo directories to display in interactive mode.");
            return;
        }

        println!("\nLaunching interactive mode...");
        let mut session = interactive::InteractiveSession::new(entries);
        
        match session.run() {
            Ok(selected_paths) => {
                if selected_paths.is_empty() {
                    println!("No directories selected for deletion.");
                    return;
                }

                // Confirm deletion
                if deletion::confirm_deletion(&selected_paths) {
                    match deletion::delete_directories(&selected_paths) {
                        Ok(report) => {
                            println!("\nDeletion complete:");
                            println!("  Successfully deleted: {}", report.successful.len());
                            println!("  Failed: {}", report.failed.len());
                            println!("  Space freed: {}", utils::format_size(report.total_freed_bytes));
                            
                            if !report.failed.is_empty() {
                                println!("\nFailed deletions:");
                                for (path, reason) in &report.failed {
                                    println!("  {}: {}", path.display(), reason);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error during deletion: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    println!("Deletion cancelled.");
                }
            }
            Err(e) => {
                eprintln!("Error in interactive mode: {}", e);
                process::exit(1);
            }
        }
    }
}
