# Unreal Engine Log Parser

**Unreal Engine Log Parser** is a Rust-based command-line tool designed to parse and filter Unreal Engine log files. It leverages the `pest` parser for efficient log parsing and `clap` for a user-friendly CLI interface.

https://crates.io/crates/unreal_log_parser

https://docs.rs/unreal_log_parser/latest/unreal_log_parser/

## Features

- **Parse Unreal Engine Logs:** Extracts structured information from log files, including timestamps, frame numbers, categories, verbosity levels, and messages.
- **Filter Logs:** Allows filtering based on verbosity levels and specific log categories.
- **Command-Line Interface:** Easy-to-use commands for parsing logs and viewing credits.
- **Error Handling:** Robust error handling to manage invalid log entries and parsing issues.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, clone the repository and build the project:

```bash
git clone https://github.com/KyrylSydorov/unreal_log_parser.git
cd unreal_log_parser
cargo build --release
```
The executable will be available in the target/release directory.

## Usage
```bash
unreal_log_parser [COMMAND]
```

## Commands

**help**: Display the help message.
```bash
unreal_log_parser help
```

**credits**: Display the program credits.
```bash
unreal_log_parser credits
```

**parse**: Parse an Unreal Engine log file with optional filters.
```bash
unreal_log_parser parse -i <input_file> -o <output_file> [OPTIONS]
```

Options:

**-i, --input <input_file>**: Path to the input log file (required).

**-o, --output <output_file>**: Path to the output file (required).

**-v, --verbosity <verbosity>**: Verbosity level to include (e.g., Fatal, Error, Warning, Display, Log, Verbose, VeryVerbose).

**-c, --category <category>**: Filter logs by a specific category.

## Examples
### Parse Logs with Verbosity Warning:

```bash
unreal_log_parser parse -i game.log -o warnings.log -v Warning
```

### Parse Logs for a Specific Category:
```bash
unreal_log_parser parse -i game.log -o core_errors.log -c LogCore
```

### Parse Logs with Verbosity and Category Filters:
```bash
unreal_log_parser parse -i game.log -o filtered.log -v Error -c LogAI
```

## Grammar Specification

The Unreal Engine Log Parser utilizes a [PEG (Parsing Expression Grammar)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) defined using the [`pest`](https://pest.rs/) parser generator. Below is a detailed breakdown of the grammar rules used to parse Unreal Engine log files.

### Overview

A typical Unreal Engine log line follows this structure:
```
[2024.04.27-12.34.56:789][  1]LogTemp: Warning: This is a warning message.
```

This line consists of:

1. **Timestamp:** `[2024.04.27-12.34.56:789]`
2. **Frame Number:** `[  1]`
3. **Category:** `LogTemp:`
4. **Verbosity:** `Warning:`
5. **Message:** `This is a warning message.`

### Grammar Rules

1. Whitespace and Newline

WHITESPACE: Defines what constitutes whitespace in the log file. It includes spaces, tabs, and newlines.

```pest
WHITESPACE = _{ " " | "\t" | NEWLINE }
```
NEWLINE: Recognizes both Unix (\n) and Windows (\r\n) newline characters.

```pest
NEWLINE = _{ "\n" | "\r\n" }
```

2. File Structure

file: Represents the entire log file. It starts at the beginning of the input (SOI) and consists of zero or more line entries, ending at the end of the input (EOI).

```pest
file = { SOI ~ line* ~ EOI }
```

3. Log Line Structure

line: Defines a single log entry. Each line may optionally start with a timestamp and frame_num, followed by mandatory category, optional verbosity, and the message. An optional newline may follow.

```pest
line = { timestamp? ~ frame_num? ~ category ~ verbosity? ~ message ~ NEWLINE? }
```

4. Timestamp Parsing

timestamp: Enclosed in square brackets, it contains the datetime.

```pest
timestamp = { "[" ~ datetime ~ "]" }
```
datetime: Breaks down the timestamp into its constituent parts: year, month, day, hour, minute, second, and millisecond.

```pest
datetime = { year ~ "." ~ month ~ "." ~ day ~ "-" ~ hour ~ "." ~ minute ~ "." ~ second ~ ":" ~ millisecond }
```
**Individual Components:**

year: Exactly four digits.

```pest
year = @{ ASCII_DIGIT{4} }
```
month, day, hour, minute, second: Exactly two digits each.

```pest
month = @{ ASCII_DIGIT{2} }
day = @{ ASCII_DIGIT{2} }
hour = @{ ASCII_DIGIT{2} }
minute = @{ ASCII_DIGIT{2} }
second = @{ ASCII_DIGIT{2} }
```
millisecond: Exactly three digits.

```pest
millisecond = @{ ASCII_DIGIT{3} }
```
5. Frame Number Parsing

frame_num: Enclosed in square brackets, it contains the frame_number.

```pest
frame_num = { "[" ~ frame_number ~ "]" }
```

frame_number: Consists of one or more digits, potentially followed by spaces or tabs. Leading and trailing whitespace is allowed.

```pest
frame_number = @{ ws? ~ ws? ~ ASCII_DIGIT+ }
```
Example Matches:
```
[1] → 1
[ 123 ] → 123
[1 2 3] → 123
```
6. Category Parsing

category: An identifier followed by a colon and optional whitespace.

```pest
category = { identifier ~ ":" ~ ws }
```
identifier: Consists of one or more alphanumeric characters, underscores, or forward slashes.

```pest
identifier = @{ (ASCII_ALPHANUMERIC | "_" | "/")+ }
```
Example Matches:
```
LogTemp:
LogRender:
```

7. Verbosity Parsing

verbosity: A predefined verbosity string followed by a colon and optional whitespace.

```pest
verbosity = { verbosity_str ~ ":" ~ ws }
```
verbosity_str: Enumerates the allowed verbosity levels.

```pest
verbosity_str = { "Verbose" | "VeryVerbose" | "Display" | "Log" | "Warning" | "Error" | "Fatal" }
```
Allowed Values:
- Verbose 
- VeryVerbose
- Display
- Log
- Warning
- Error
- Fatal
8. Message Parsing
message: Captures the rest of the line as the log message. It includes any character except for a newline.

## Credits
Made by Kyryl Sydorov. Visit my GitHub.

## License
This project is licensed under the MIT License.