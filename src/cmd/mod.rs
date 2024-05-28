use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator};
use cli::{Cli, Command, PrintNowOptions};
use et::{parse, print_now};

pub mod cli;
pub mod et;

pub enum SecondsResolution {
    Secs,
    Millis,
    Micros,
    Nanos,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
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
            Command::Completions { shell } => print_completions(shell, &mut Cli::command()),
        }
    } else {
        print_now(PrintNowOptions::default(), SecondsResolution::Millis)
    }
}
