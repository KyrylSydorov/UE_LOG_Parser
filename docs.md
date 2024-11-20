# Unreal Engine Log Parser

**Unreal Engine Log Parser** is a Rust-based command-line tool designed to parse and filter Unreal Engine log files. It leverages the `pest` parser for efficient log parsing and `clap` for a user-friendly CLI interface.

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