extern crate chrono;

use chrono::{FixedOffset, Timelike, Utc};

fn main() {
    // Swatch Internet Time standardizes on UTC+1
    let offset = FixedOffset::east_opt(3600).unwrap();
    let now = Utc::now().with_timezone(&offset);

    let beats = (now.hour() * 3600 + now.minute() * 60 + now.second()) as f64 / 86.4;

    println!("{} @{:06.2}", now.date_naive(), beats);
}
