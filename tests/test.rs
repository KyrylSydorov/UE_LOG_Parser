// Kyryl Sydorov, 2024

use unreal_log_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_entry_with_verbosity() {
        let log_line = "[2024.04.27-12.34.56:789][  1]LogTemp: Warning: This is a warning message.";

        let parsed = LogEntry::parse(log_line).expect("Failed to parse log line");

        let expected = LogEntry {
            timestamp: Timestamp {
                year: 2024,
                month: 4,
                day: 27,
                hour: 12,
                minute: 34,
                second: 56,
                millisecond: 789,
            },
            frame_num: 1,
            category: "LogTemp".to_string(),
            verbosity: Verbosity::Warning,
            message: "This is a warning message.".to_string(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_log_entry_without_verbosity() {
        let log_line = "[2024.04.27-12.34.56:789][  1]LogTemp: This is a log message.";

        let parsed = LogEntry::parse(log_line).expect("Failed to parse log line");

        let expected = LogEntry {
            timestamp: Timestamp {
                year: 2024,
                month: 4,
                day: 27,
                hour: 12,
                minute: 34,
                second: 56,
                millisecond: 789,
            },
            frame_num: 1,
            category: "LogTemp".to_string(),
            verbosity: Verbosity::Log,
            message: "This is a log message.".to_string(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_log_minimal() {
        let log_line = "LogCore: Very small log";

        let parsed = LogEntry::parse(log_line).expect("Failed to parse log line");

        let expected = LogEntry {
            timestamp: Timestamp::new(),
            frame_num: 0,
            category: "LogCore".to_string(),
            verbosity: Verbosity::Log,
            message: "Very small log".to_string(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_invalid_string() {
        let log_line = "This is not a log line";
        let parsed = LogEntry::parse(log_line);
        assert_eq!(parsed, Err(UnrealLogParserError::ParseError));
    }

    #[test]
    fn test_invalid_timestamp() {
        let log_line = "[2024.04.27][  1]LogTemp: This is a log message.";
        let parsed = LogEntry::parse(log_line);
        assert_eq!(parsed, Err(UnrealLogParserError::ParseError));
    }

    #[test]
    fn test_invalid_verbosity() {
        let log_line = "[2024.04.27-12.34.56:789][  1]LogTemp: InvalidVerbosity: This is a log message.";
        let parsed = LogEntry::parse(log_line);
        assert!(parsed.is_ok()); // Invalid verbosity is ignored (treated like part of a message)
    }

    #[test]
    fn test_file() {
        let file_path = "tests/Data/SmallTestFile.log";
        let mut file = LogFile::new(file_path.parse().unwrap());

        let parsed_result = file.parse();
        assert!(parsed_result.is_ok());

        assert_eq!(file.entries[0], LogEntry {
            timestamp: Timestamp {
                year: 2024,
                month: 10,
                day: 4,
                hour: 19,
                minute: 09,
                second: 47,
                millisecond: 074,
            },
            frame_num: 0,
            category: "LogAssetRegistry".to_string(),
            verbosity: Verbosity::Display,
            message: "Triggering cache save on discovery complete".to_string(),
        });
    }

    #[test]
    fn test_no_file() {
        let file_path = "tests/Data/NonExistentFile.log";
        let mut file = LogFile::new(file_path.parse().unwrap());

        let parsed_result = file.parse();
        assert_eq!(parsed_result, Err(UnrealLogParserError::NoSuchFile));
    }

    #[test]
    fn test_empty_file() {
        let file_path = "tests/Data/EmptyFile.log";
        let mut file = LogFile::new(file_path.parse().unwrap());

        let parsed_result = file.parse();
        assert_eq!(parsed_result, Err(UnrealLogParserError::NoLogEntriesFound));
    }

    #[test]
    fn test_huge_file() {
        let file_path = "tests/Data/TestLogFile.log";
        let mut file = LogFile::new(file_path.parse().unwrap());

        let parsed_result = file.parse();
        assert!(parsed_result.is_ok());
    }
}
