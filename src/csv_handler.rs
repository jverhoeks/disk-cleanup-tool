use crate::scanner::{DirectoryEntry, EntryType};
use csv::{Reader, Writer};
use std::fs::File;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsvError {
    #[error("Missing required column: {0}")]
    MissingColumn(String),

    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
}

pub fn write_csv(entries: &[DirectoryEntry], path: &Path) -> Result<(), CsvError> {
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);

    // Write header
    writer.write_record(&["path", "files", "size_bytes", "cumulative_files", "cumulative_size_bytes", "type"])?;

    // Write entries
    for entry in entries {
        let entry_type = match entry.entry_type {
            EntryType::Temp => "temp",
            EntryType::Normal => "normal",
        };

        writer.write_record(&[
            entry.path.to_string_lossy().as_ref(),
            &entry.file_count.to_string(),
            &entry.size_bytes.to_string(),
            &entry.cumulative_file_count.to_string(),
            &entry.cumulative_size_bytes.to_string(),
            entry_type,
        ])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn read_csv(path: &Path) -> Result<Vec<DirectoryEntry>, CsvError> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);

    // Verify headers
    let headers = reader.headers()?;
    let required = ["path", "files", "size_bytes", "type"];
    for req in &required {
        if !headers.iter().any(|h| h == *req) {
            return Err(CsvError::MissingColumn(req.to_string()));
        }
    }

    // Check if we have cumulative columns (new format)
    let has_cumulative = headers.iter().any(|h| h == "cumulative_files");

    let mut entries = Vec::new();

    for (line_num, result) in reader.records().enumerate() {
        let record = result.map_err(|e| CsvError::ParseError {
            line: line_num + 2, // +2 because line 1 is header and enumerate starts at 0
            message: e.to_string(),
        })?;

        let expected_cols = if has_cumulative { 6 } else { 4 };
        if record.len() < expected_cols {
            return Err(CsvError::ParseError {
                line: line_num + 2,
                message: format!("Expected {} columns, found {}", expected_cols, record.len()),
            });
        }

        let path = record[0].into();
        let file_count = record[1].parse::<u64>().map_err(|e| CsvError::ParseError {
            line: line_num + 2,
            message: format!("Invalid file count: {}", e),
        })?;
        let size_bytes = record[2].parse::<u64>().map_err(|e| CsvError::ParseError {
            line: line_num + 2,
            message: format!("Invalid size: {}", e),
        })?;

        let (cumulative_file_count, cumulative_size_bytes, type_idx) = if has_cumulative {
            let cum_files = record[3].parse::<u64>().map_err(|e| CsvError::ParseError {
                line: line_num + 2,
                message: format!("Invalid cumulative file count: {}", e),
            })?;
            let cum_size = record[4].parse::<u64>().map_err(|e| CsvError::ParseError {
                line: line_num + 2,
                message: format!("Invalid cumulative size: {}", e),
            })?;
            (cum_files, cum_size, 5)
        } else {
            // Old format: use direct values as cumulative
            (file_count, size_bytes, 3)
        };

        let entry_type = match &record[type_idx] {
            "temp" => EntryType::Temp,
            "normal" => EntryType::Normal,
            other => {
                return Err(CsvError::ParseError {
                    line: line_num + 2,
                    message: format!("Invalid entry type: {}", other),
                })
            }
        };

        entries.push(DirectoryEntry {
            path,
            file_count,
            size_bytes,
            cumulative_file_count,
            cumulative_size_bytes,
            entry_type,
        });
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::EntryType;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_and_read_csv() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let entries = vec![
            DirectoryEntry {
                path: PathBuf::from("/home/user/project"),
                file_count: 100,
                size_bytes: 1024000,
                cumulative_file_count: 5100,
                cumulative_size_bytes: 525312000,
                entry_type: EntryType::Normal,
            },
            DirectoryEntry {
                path: PathBuf::from("/home/user/project/node_modules"),
                file_count: 5000,
                size_bytes: 524288000,
                cumulative_file_count: 5000,
                cumulative_size_bytes: 524288000,
                entry_type: EntryType::Temp,
            },
        ];

        // Write CSV
        write_csv(&entries, path).unwrap();

        // Read CSV back
        let loaded = read_csv(path).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].path, PathBuf::from("/home/user/project"));
        assert_eq!(loaded[0].file_count, 100);
        assert_eq!(loaded[0].size_bytes, 1024000);
        assert_eq!(loaded[0].cumulative_file_count, 5100);
        assert_eq!(loaded[0].cumulative_size_bytes, 525312000);
        assert_eq!(loaded[0].entry_type, EntryType::Normal);

        assert_eq!(loaded[1].path, PathBuf::from("/home/user/project/node_modules"));
        assert_eq!(loaded[1].file_count, 5000);
        assert_eq!(loaded[1].size_bytes, 524288000);
        assert_eq!(loaded[1].cumulative_file_count, 5000);
        assert_eq!(loaded[1].cumulative_size_bytes, 524288000);
        assert_eq!(loaded[1].entry_type, EntryType::Temp);
    }

    #[test]
    fn test_read_malformed_csv() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write malformed CSV (missing column)
        std::fs::write(path, "path,files,size_bytes\n/test,10,100\n").unwrap();

        let result = read_csv(path);
        assert!(matches!(result, Err(CsvError::MissingColumn(_))));
    }

    #[test]
    fn test_read_old_format_csv() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write old format CSV (without cumulative columns)
        std::fs::write(path, "path,files,size_bytes,type\n/test,10,100,normal\n").unwrap();

        let result = read_csv(path).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file_count, 10);
        assert_eq!(result[0].size_bytes, 100);
        // Old format should use direct values as cumulative
        assert_eq!(result[0].cumulative_file_count, 10);
        assert_eq!(result[0].cumulative_size_bytes, 100);
    }

    #[test]
    fn test_read_invalid_number() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write CSV with invalid number (old format)
        std::fs::write(path, "path,files,size_bytes,type\n/test,abc,100,normal\n").unwrap();

        let result = read_csv(path);
        assert!(matches!(result, Err(CsvError::ParseError { .. })));
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use crate::scanner::EntryType;
    use proptest::prelude::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    // Feature: disk-cleanup-tool, Property 9: CSV type labeling
    // Validates: Requirements 3.3
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_csv_type_labeling(
            path in "[a-z/]{1,30}",
            file_count in 0u64..1000,
            size_bytes in 0u64..1000000,
            is_temp in prop::bool::ANY
        ) {
            let temp_file = NamedTempFile::new().unwrap();
            let csv_path = temp_file.path();

            let entry_type = if is_temp { EntryType::Temp } else { EntryType::Normal };
            let entries = vec![DirectoryEntry {
                path: PathBuf::from(path),
                file_count,
                size_bytes,
                cumulative_file_count: file_count,
                cumulative_size_bytes: size_bytes,
                entry_type,
            }];

            write_csv(&entries, csv_path).unwrap();

            // Read the CSV as text and check type column
            let content = std::fs::read_to_string(csv_path).unwrap();
            let lines: Vec<&str> = content.lines().collect();
            
            prop_assert!(lines.len() >= 2); // header + data
            
            let data_line = lines[1];
            if is_temp {
                prop_assert!(data_line.ends_with(",temp"));
            } else {
                prop_assert!(data_line.ends_with(",normal"));
            }
        }

        // Feature: disk-cleanup-tool, Property 10: CSV size formatting
        // Validates: Requirements 3.5
        #[test]
        fn test_csv_size_as_integer(
            size_bytes in 0u64..1000000000
        ) {
            let temp_file = NamedTempFile::new().unwrap();
            let csv_path = temp_file.path();

            let entries = vec![DirectoryEntry {
                path: PathBuf::from("/test"),
                file_count: 1,
                size_bytes,
                cumulative_file_count: 1,
                cumulative_size_bytes: size_bytes,
                entry_type: EntryType::Normal,
            }];

            write_csv(&entries, csv_path).unwrap();

            let content = std::fs::read_to_string(csv_path).unwrap();
            let lines: Vec<&str> = content.lines().collect();
            let data_line = lines[1];
            let parts: Vec<&str> = data_line.split(',').collect();
            
            // Size should be third column and parse as integer
            let size_str = parts[2];
            prop_assert!(size_str.parse::<u64>().is_ok());
            prop_assert_eq!(size_str.parse::<u64>().unwrap(), size_bytes);
        }

        // Feature: disk-cleanup-tool, Property 11: CSV round-trip consistency
        // Validates: Requirements 6.2
        #[test]
        fn test_csv_roundtrip(
            num_entries in 1usize..10,
            seed in 0u64..1000
        ) {
            let temp_file = NamedTempFile::new().unwrap();
            let csv_path = temp_file.path();

            // Generate entries
            let mut entries = Vec::new();
            for i in 0..num_entries {
                let file_count = (seed + i as u64) % 100;
                let size_bytes = (seed * (i as u64 + 1)) % 10000;
                entries.push(DirectoryEntry {
                    path: PathBuf::from(format!("/path{}", i)),
                    file_count,
                    size_bytes,
                    cumulative_file_count: file_count + i as u64,
                    cumulative_size_bytes: size_bytes + (i as u64 * 100),
                    entry_type: if i % 2 == 0 { EntryType::Temp } else { EntryType::Normal },
                });
            }

            // Write and read back
            write_csv(&entries, csv_path).unwrap();
            let loaded = read_csv(csv_path).unwrap();

            prop_assert_eq!(entries.len(), loaded.len());
            
            for (original, loaded) in entries.iter().zip(loaded.iter()) {
                prop_assert_eq!(&original.path, &loaded.path);
                prop_assert_eq!(original.file_count, loaded.file_count);
                prop_assert_eq!(original.size_bytes, loaded.size_bytes);
                prop_assert_eq!(original.cumulative_file_count, loaded.cumulative_file_count);
                prop_assert_eq!(original.cumulative_size_bytes, loaded.cumulative_size_bytes);
                prop_assert_eq!(original.entry_type, loaded.entry_type);
            }
        }

        // Feature: disk-cleanup-tool, Property 20: Malformed CSV error handling
        // Validates: Requirements 6.4
        #[test]
        fn test_malformed_csv_errors(
            bad_number in "[a-z]{1,10}"
        ) {
            let temp_file = NamedTempFile::new().unwrap();
            let csv_path = temp_file.path();

            // Write CSV with invalid number
            let content = format!("path,files,size_bytes,type\n/test,{},100,normal\n", bad_number);
            std::fs::write(csv_path, content).unwrap();

            let result = read_csv(csv_path);
            prop_assert!(result.is_err());
        }
    }
}
