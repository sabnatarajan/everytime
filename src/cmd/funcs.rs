use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;

use super::{cli::PrintNowOptions, SecondsResolution};

pub fn print_now(options: PrintNowOptions, seconds_resolution: SecondsResolution) {
    let timezone = match options.tz {
        Some(tz) => Tz::from_str_insensitive(tz.as_str()).unwrap_or(Tz::UTC),
        None => Tz::UTC,
    };

    let datetime = Utc::now().with_timezone(&timezone);

    if options.as_str {
        println!(
            "{}",
            datetime.to_rfc3339_opts(chrono::SecondsFormat::from(seconds_resolution), true)
        );
    } else {
        println!("{}", datetime.timestamp_with_resolution(seconds_resolution));
    }
}

impl From<SecondsResolution> for chrono::SecondsFormat {
    fn from(value: SecondsResolution) -> Self {
        match value {
            SecondsResolution::Secs => chrono::SecondsFormat::Secs,
            SecondsResolution::Millis => chrono::SecondsFormat::Millis,
            SecondsResolution::Micros => chrono::SecondsFormat::Micros,
            SecondsResolution::Nanos => chrono::SecondsFormat::Nanos,
        }
    }
}

trait TimestampWithResolution {
    fn timestamp_with_resolution(&self, resolution: SecondsResolution) -> i64;
}

impl TimestampWithResolution for DateTime<Tz> {
    fn timestamp_with_resolution(&self, resolution: SecondsResolution) -> i64 {
        match resolution {
            SecondsResolution::Millis => self.timestamp_millis(),
            SecondsResolution::Secs => self.timestamp(),
            SecondsResolution::Micros => self.timestamp_micros(),
            SecondsResolution::Nanos => self.timestamp_nanos_opt().unwrap(),
        }
    }
}

pub fn parse(timestamp: String) {
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
