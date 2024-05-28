use clap::Parser;
use cli::{Cli, Command, PrintNowOptions};
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

    if let Some(cmd) = args.command {
        match cmd {
            Command::PrintSecs { options } => print_now(options, SecondsResolution::Secs),
            Command::PrintMillis { options } => print_now(options, SecondsResolution::Millis),
            Command::PrintMicros { options } => print_now(options, SecondsResolution::Micros),
            Command::PrintNanos { options } => print_now(options, SecondsResolution::Nanos),
            Command::Parse { parse_str } => parse(parse_str),
        }
    } else {
        print_now(PrintNowOptions::default(), SecondsResolution::Millis)
    }
}
