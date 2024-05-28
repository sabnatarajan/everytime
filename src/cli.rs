use chrono::{DateTime, Local};
use chrono_tz::Tz;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Everytime {
    /// Display the current time at a timezone
    Now {
        /// IANA Zone Id (Eg: America/New_York). Defaults to local time
        tz: Option<String>,

        /// Display as time since the Unix Epoch
        #[structopt(short)]
        epoch: bool,
    },
    /// Parse a timestamp to the current local time
    Parse {
        /// The timestamp (in milliseconds) to parse
        parse_str: String,
    },
}

fn now(tz: Option<String>, epoch: bool) -> String {
    let tz = match tz {
        Some(s) => s,
        None => "".to_string(),
    };

    let t = Tz::from_str_insensitive(tz.as_str());
    let now = Local::now();
    match t {
        Ok(t) => match epoch {
            true => now.with_timezone(&t).timestamp().to_string(),
            false => now.with_timezone(&t).to_string(),
        },
        Err(_) => match epoch {
            true => now.timestamp().to_string(),
            false => now.to_string(),
        },
    }
}

fn parse(timestamp: String) {
    let ts = timestamp.parse::<i64>().expect("Unable to parse");
    let mut local_dt = None;

    // h/t epochconverter.com
    if ts >= 1e8 as i64 && ts < 18e7 as i64 {
        eprintln!("Expected a more recent date. You are missing 1 digit");
    } else if ts >= 1e14 as i64 || ts <= -1e14 as i64 {
        local_dt = DateTime::from_timestamp_micros(ts);
    } else if ts >= 1e11 as i64 || ts <= -3e10 as i64 {
        local_dt = DateTime::from_timestamp_millis(ts);
    } else {
        local_dt = DateTime::from_timestamp(ts, 0);
    }

    if let Some(dt) = local_dt {
        println!("{}", DateTime::<Local>::from(dt));
    } else {
        eprintln!("unknown error while parsing");
    }
}

pub fn execute() {
    let args = Everytime::from_args();

    match args {
        Everytime::Now { tz, epoch } => println!("{}", now(tz, epoch)),
        Everytime::Parse { parse_str } => parse(parse_str),
    }
}
