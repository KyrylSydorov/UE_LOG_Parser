# Unreal Engine Log Parser

**Unreal Engine Log Parser** is a Rust-based command-line tool designed to parse and filter Unreal Engine log files. It leverages the `pest` parser for efficient log parsing and `clap` for a user-friendly CLI interface.

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

## Example Log Entry
``` less
[2024.04.27-12.34.56:789][  1]LogTemp: Warning: This is a warning message.
```

## Credits
Made by Kyryl Sydorov. Visit my GitHub.

## License
This project is licensed under the MIT License.