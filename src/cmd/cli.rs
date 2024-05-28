use chrono_tz::Tz;
use clap::Parser;
use clap_complete::shells;

#[derive(clap::Args, Debug)]
pub struct PrintNowOptions {
    /// IANA Zone Id (Eg: America/New_York). Defaults to local time
    pub tz: Option<String>,

    /// Display as a datetime string
    #[arg(short = 's', long = "str")]
    pub as_str: bool,
}

impl Default for PrintNowOptions {
    fn default() -> Self {
        Self {
            tz: Some(Tz::UTC.to_string()),
            as_str: false,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "et", about = "Work with datetimes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Display the current time at a timezone
    #[command(name = "secs")]
    PrintSecs {
        #[command(flatten)]
        options: PrintNowOptions,
    },

    /// Display the current time at a timezone in milliseconds
    #[command(name = "millis")]
    PrintMillis {
        #[command(flatten)]
        options: PrintNowOptions,
    },

    /// Display the current time at a timezone in microseconds
    #[command(name = "micros")]
    PrintMicros {
        #[command(flatten)]
        options: PrintNowOptions,
    },

    /// Display the current time at a timezone in nanoseconds
    #[command(name = "nanos")]
    PrintNanos {
        #[command(flatten)]
        options: PrintNowOptions,
    },

    /// Parse a timestamp to the current local time
    #[command(name = "parse")]
    Parse {
        /// The timestamp (in milliseconds) to parse
        parse_str: String,
    },

    /// Generate completions
    #[command(name = "completions")]
    Completions {
        /// The shell to generate completions for
        shell: shells::Shell,
    },
}
