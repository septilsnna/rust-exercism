use chrono::{DateTime, Utc};
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start_date: DateTime<Utc>) -> DateTime<Utc> {
    start_date + Duration::seconds(1_000_000_000)
   // unimplemented!("What time is a gigasecond later than {}", start_date);
}
