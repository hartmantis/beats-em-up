extern crate chrono;
extern crate clap;

use chrono::{FixedOffset, Timelike, Utc};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    // Optionally print the date as well as the time.
    // Helpful for that period where it's already "tomorrow" in SIT.
    #[arg(short, long)]
    date: bool,
}

fn main() {
    let args = Args::parse();

    // Swatch Internet Time standardizes on UTC+1
    let offset = FixedOffset::east_opt(3600).unwrap();
    let now = Utc::now().with_timezone(&offset);

    let beats = (now.hour() * 3600 + now.minute() * 60 + now.second()) as f64 / 86.4;

    if args.date == true {
        println!("{} @{:06.2}", now.date_naive(), beats);
    } else {
        println!("@{:06.2}", beats);
    }
}
