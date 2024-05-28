use clap::Parser;

#[derive(clap::Args, Debug)]
pub struct PrintNowOptions {
    /// IANA Zone Id (Eg: America/New_York). Defaults to local time
    pub tz: Option<String>,

    /// Display as a datetime string
    #[arg(short = 's', long = "str")]
    pub as_str: bool,
}

#[derive(Debug, Parser)]
#[command(name = "et", about = "Work with datetimes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
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
}
