use clap::Parser;
use cli::{Cli, Commands};
use funcs::{parse, print_now};

pub mod cli;
pub mod funcs;

pub enum SecondsResolution {
    Secs,
    Millis,
    Micros,
    Nanos,
}

pub fn execute() {
    let args = Cli::parse();

    match args.command {
        Commands::PrintSecs { options } => print_now(options, SecondsResolution::Secs),
        Commands::PrintMillis { options } => print_now(options, SecondsResolution::Millis),
        Commands::PrintMicros { options } => print_now(options, SecondsResolution::Micros),
        Commands::PrintNanos { options } => print_now(options, SecondsResolution::Nanos),
        Commands::Parse { parse_str } => parse(parse_str),
    }
}
