// Kyryl Sydorov, 2024

use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use std::io::Write;
use unreal_log_parser::*;

const APP_NAME: &str = "Unreal Engine Log Parser";
const APP_SHORT_NAME: &str = "unreal_log_parser";
const APP_VERSION: &str = "1.1.1";

const COMMAND_HELP: &str = "help";

const COMMAND_CREDITS: &str = "credits";
const COMMAND_CREDITS_INFO: &str = "Print the credits of the program";

const COMMAND_PARSE: &str = "parse";
const COMMAND_PARSE_INFO: &str = "Parse an Unreal Engine log file";

const COMMAND_PARSE_INPUT: &str = "input";
const COMMAND_PARSE_OUTPUT: &str = "output";
const COMMAND_PARSE_VERBOSITY: &str = "verbosity";
const COMMAND_PARSE_CATEGORY: &str = "category";

fn main() -> Result<()> {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .subcommand(Command::new(COMMAND_CREDITS).about(COMMAND_CREDITS_INFO))
        .subcommand(
            Command::new(COMMAND_PARSE)
                .about(COMMAND_PARSE_INFO)
                .arg(
                    Arg::new(COMMAND_PARSE_INPUT)
                        .short('i')
                        .help("Path to the input file")
                        .required(true),
                )
                .arg(
                    Arg::new(COMMAND_PARSE_OUTPUT)
                        .short('o')
                        .help("Path to the output file")
                        .required(true),
                )
                .arg(
                    Arg::new(COMMAND_PARSE_VERBOSITY)
                        .short('v')
                        .help(
                            "Verbosity level to include\n\
                            \tFatal\n\
                            \tError\n\
                            \tWarning\n\
                            \tDisplay\n\
                            \tLog\n\
                            \tVerbose\n\
                            \tVeryVerbose\n",
                        )
                        .required(false),
                )
                .arg(
                    Arg::new(COMMAND_PARSE_CATEGORY)
                        .short('c')
                        .help("Look for logs with a specific category")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some((COMMAND_CREDITS, _)) => {
            print_credits();
        }
        Some((COMMAND_PARSE, parse_matches)) => {
            process(parse_matches)?;
        }
        _ => {
            println!("Unknown command!");
            println!(
                "Use \"{} {}\" to see the list of available commands",
                APP_SHORT_NAME, COMMAND_HELP
            );
        }
    }

    Ok(())
}

fn print_credits() {
    println!("{} Credits:", APP_NAME);
    println!("Made by Kyryl Sydorov");
    println!("Visit my GitHub: https://github.com/KyrylSydorov");
    println!("Version {}", APP_VERSION);
}

fn process(matches: &ArgMatches) -> Result<()> {
    let input = matches.get_one::<String>(COMMAND_PARSE_INPUT).unwrap();
    let output = matches.get_one::<String>(COMMAND_PARSE_OUTPUT).unwrap();
    let verbosity = matches.get_one::<String>(COMMAND_PARSE_VERBOSITY);
    let category = matches.get_one::<String>(COMMAND_PARSE_CATEGORY);

    let mut file = LogFile::new(input.clone());
    let parse_result = file.parse();

    match parse_result {
        Ok(_) => {
            println!("Parsed successfully!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let log_entries = file.entries.iter().filter(|entry| {
        if let Some(verbosity) = verbosity {
            if entry.verbosity.to_string() != verbosity.clone() {
                return false;
            }
        }

        if let Some(category) = category {
            if entry.category != category.clone() {
                return false;
            }
        }

        true
    });

    if log_entries.clone().count() == 0 {
        println!("No log entries found with the specified criteria");
        return Ok(());
    }

    let mut output_file = std::fs::File::create(output)?;
    for entry in log_entries {
        writeln!(&mut output_file, "[\n{}\n]", entry)?;
    }

    Ok(())
}
