// Kyryl Sydorov, 2024

pub use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct UnrealLogParser;

/// Example log:
/// [2024.04.27-12.34.56:789][  1]LogTemp: Warning: This is a warning message.
#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub timestamp: Timestamp,
    pub frame_num: u32,
    pub category: String,
    pub verbosity: Verbosity,
    pub message: String,
}

/// Example timestamp:
/// [2024.04.27-12.34.56:789]
#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}

#[derive(Debug, PartialEq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
    Display,
    Log,
    Warning,
    Error,
    Fatal,
}

pub const VERBOSITY_VERY_VERBOSE: &str = "VeryVerbose";
pub const VERBOSITY_VERBOSE: &str = "Verbose";
pub const VERBOSITY_LOG: &str = "Log";
pub const VERBOSITY_DISPLAY: &str = "Display";
pub const VERBOSITY_WARNING: &str = "Warning";
pub const VERBOSITY_ERROR: &str = "Error";
pub const VERBOSITY_FATAL: &str = "Fatal";

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let verbosity_str = match self {
            Verbosity::VeryVerbose => VERBOSITY_VERY_VERBOSE,
            Verbosity::Verbose => VERBOSITY_VERBOSE,
            Verbosity::Display => VERBOSITY_DISPLAY,
            Verbosity::Log => VERBOSITY_LOG,
            Verbosity::Warning => VERBOSITY_WARNING,
            Verbosity::Error => VERBOSITY_ERROR,
            Verbosity::Fatal => VERBOSITY_FATAL,
        };
        write!(f, "{}", verbosity_str)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Date: {:04}.{:02}.{:02} Time: {:02}.{:02}.{:02} {:03}ms",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.millisecond
        )
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Timestamp: {} \nFrame: {} \nCategory: {} \nVerbosity: {} \nMessage: {}",
            self.timestamp, self.frame_num, self.category, self.verbosity, self.message
        )
    }
}

impl Verbosity {
    fn from_str(s: &str) -> Result<Verbosity, UnrealLogParserError> {
        match s {
            VERBOSITY_VERY_VERBOSE => Ok(Verbosity::VeryVerbose),
            VERBOSITY_VERBOSE => Ok(Verbosity::Verbose),
            VERBOSITY_DISPLAY => Ok(Verbosity::Display),
            VERBOSITY_LOG => Ok(Verbosity::Log),
            VERBOSITY_WARNING => Ok(Verbosity::Warning),
            VERBOSITY_ERROR => Ok(Verbosity::Error),
            VERBOSITY_FATAL => Ok(Verbosity::Fatal),
            _ => Err(UnrealLogParserError::InvalidVerbosity),
        }
    }
}

use pest::iterators::Pair;

#[derive(Debug, Error, PartialEq)]
pub enum UnrealLogParserError {
    #[error("No such file")]
    NoSuchFile,
    #[error("Generic parse error")]
    ParseError,
    #[error("No log entries found")]
    NoLogEntriesFound,
    #[error("Invalid verbosity")]
    InvalidVerbosity,
    #[error("Invalid category")]
    InvalidCategory,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Invalid frame number")]
    InvalidFrameNumber,
}

impl LogEntry {
    pub fn parse(input: &str) -> Result<LogEntry, UnrealLogParserError> {
        let mut pairs = UnrealLogParser::parse(Rule::line, input)
            .map_err(|_| UnrealLogParserError::ParseError)?;
        let inner = pairs
            .next()
            .ok_or(UnrealLogParserError::NoLogEntriesFound)?
            .into_inner();

        let mut result: LogEntry = LogEntry::new();

        for pair in inner {
            match pair.as_rule() {
                Rule::timestamp => {
                    result.timestamp = Timestamp::from_pair(pair)?;
                }
                Rule::frame_num => {
                    let frame_pair = pair
                        .into_inner()
                        .next()
                        .ok_or(UnrealLogParserError::InvalidFrameNumber)?;
                    let frame_num_str = frame_pair.as_str().replace(" ", "");
                    let frame_num = frame_num_str
                        .parse::<u32>()
                        .map_err(|_| UnrealLogParserError::InvalidFrameNumber)?;
                    result.frame_num = frame_num;
                }
                Rule::category => {
                    let category_pair = pair
                        .into_inner()
                        .next()
                        .ok_or(UnrealLogParserError::InvalidCategory)?;
                    result.category = category_pair.as_str().to_string();
                }
                Rule::verbosity => {
                    let verbosity_pair = pair
                        .into_inner()
                        .next()
                        .ok_or(UnrealLogParserError::InvalidVerbosity)?;
                    result.verbosity = Verbosity::from_str(verbosity_pair.as_str())?;
                }
                Rule::message => {
                    result.message = pair.as_str().to_string();
                }
                _ => {
                    return Err(UnrealLogParserError::ParseError);
                }
            }
        }

        Ok(result)
    }

    pub fn new() -> LogEntry {
        LogEntry {
            timestamp: Timestamp::new(),
            frame_num: 0,
            category: "".to_string(),
            verbosity: Verbosity::Log,
            message: "".to_string(),
        }
    }
}

impl Timestamp {
    fn from_pair(pair: Pair<Rule>) -> Result<Timestamp, UnrealLogParserError> {
        let mut inner = pair
            .into_inner()
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .into_inner();

        let year = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let month = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let day = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let hour = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let minute = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let second = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        let millisecond = inner
            .next()
            .ok_or(UnrealLogParserError::InvalidTimestamp)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| UnrealLogParserError::InvalidTimestamp)?;
        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        })
    }

    pub fn new() -> Timestamp {
        Timestamp {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    }
}

pub struct LogFile {
    pub entries: Vec<LogEntry>,
    pub path: String,
}

impl LogFile {
    pub fn new(path: String) -> LogFile {
        LogFile {
            entries: Vec::new(),
            path,
        }
    }

    pub fn parse(&mut self) -> Result<(), UnrealLogParserError> {
        let contents =
            std::fs::read_to_string(&self.path).map_err(|_| UnrealLogParserError::NoSuchFile)?;
        let lines = contents.lines();

        match lines.count() {
            0 => return Err(UnrealLogParserError::NoLogEntriesFound),
            _ => {
                for line in contents.lines() {
                    let entry = LogEntry::parse(line);

                    match entry {
                        Ok(_) => {
                            self.entries.push(entry?);
                        }
                        Err(_) => {
                            println!("Error parsing line: {}", line);
                        }
                    }
                }
            }
        }

        if self.entries.is_empty() {
            return Err(UnrealLogParserError::NoLogEntriesFound);
        }

        Ok(())
    }
}
