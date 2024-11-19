// Kyryl Sydorov, 2024

use anyhow::Result;
use clap::{Command, Arg, ArgMatches};

const APP_NAME: &str = "Unreal Engine Log Parser";
const APP_SHORT_NAME: &str = "UnrealLogParser";
const APP_VERSION: &str = "1.0.0";

const COMMAND_HELP: &str = "help";

const COMMAND_CREDITS: &str = "credits";
const COMMAND_CREDITS_INFO: &str = "Print the credits of the program";

const COMMAND_PARSE: &str = "parse";
const COMMAND_PARSE_INFO: &str = "Parse an Unreal Engine log file";

//const COMMAND_PARSE_VERBOSITY_FATAL: &str = "Fatal";
//const COMMAND_PARSE_VERBOSITY_ERROR: &str = "Error";
//const COMMAND_PARSE_VERBOSITY_WARNING: &str = "Warning";
//const COMMAND_PARSE_VERBOSITY_DISPLAY: &str = "Display";
//const COMMAND_PARSE_VERBOSITY_LOG: &str = "Log";
//const COMMAND_PARSE_VERBOSITY_VERBOSE: &str = "Verbose";
//const COMMAND_PARSE_VERBOSITY_VERY_VERBOSE: &str = "VeryVerbose";

fn main() -> Result<()> {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .subcommand(
            Command::new(COMMAND_CREDITS)
                .about(COMMAND_CREDITS_INFO)
        )
        .subcommand(
            Command::new(COMMAND_PARSE)
                .about(COMMAND_PARSE_INFO)
                .arg(
                    Arg::new("input")
                        .short('i')
                        .help("Path to the input file")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .help("Path to the output file")
                        .required(true),
                )
                .arg(
                    Arg::new("verbosity")
                        .short('v')
                        .help("Minimum verbosity level to include\n\
                            \tFatal\n\
                            \tError\n\
                            \tWarning\n\
                            \tDisplay\n\
                            \tLog\n\
                            \tVerbose\n\
                            \tVeryVerbose\n")
                        .required(false),
                )
                .arg(
                    Arg::new("category")
                        .short('c')
                        .help("Look for logs with a specific category")
                        .required(false),
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some((COMMAND_CREDITS, _)) => {
            print_credits();
        },
        Some((COMMAND_PARSE, parse_matches)) => {
            process(parse_matches)?;
        },
        _ => {
            println!("Unknown command!");
            println!("Use \"{} {}\" to see the list of available commands", APP_SHORT_NAME, COMMAND_HELP);
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

fn process(matches: &ArgMatches) -> Result<()> {}