use structopt::StructOpt;
use chrono::{Local, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

#[derive(Debug, StructOpt)]
enum Everytime {
    /// Display the current time at a timezone
    Now {
        /// IANA Zone Id (Eg: America/New_York). Defaults to local time
        tz: Option<String>,

        /// Display as time since the Unix Epoch
        #[structopt(short)]
        epoch: bool
    },
    /// Parse a timestamp to the current local time
    Parse {
        /// The timestamp (in milliseconds) to parse
        parse_str: String
    },
}

fn now(tz: Option<String>, epoch: bool) -> String {
    let tz = match tz {
        Some(s) => s,
        None => "".to_string()
    };

    let t:Result<Tz,String> = tz.parse();
    let now = Local::now();
    match t {
        Ok(t) => match epoch {
            true => now.with_timezone(&t).timestamp().to_string(),
            false => now.with_timezone(&t).to_string()
        },
        Err(_) => match epoch {
            true => now.timestamp().to_string(),
            false => now.to_string()
        }
    }
}

fn parse(parse_str: String) {
    let parse_millis: Result<i64, _> = parse_str.parse();
    match parse_millis {
        Ok(millis) => {
            let nsecs = 0;
            let ndt = NaiveDateTime::from_timestamp(millis, nsecs);
            let ldt = Local.from_utc_datetime(&ndt).to_rfc3339();
            println!("{}", ldt);
        },
        Err(_) => eprintln!("Could not parse")
    }
}

pub fn execute() {
    let args = Everytime::from_args();

    match args {
        Everytime::Now { tz, epoch } => println!("{}", now(tz, epoch)),
        Everytime::Parse { parse_str } => parse(parse_str),
    }
}
