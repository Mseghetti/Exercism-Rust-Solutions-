use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const gigasecond: i64 = 1_000_000_000;
    let duration = Duration::seconds(gigasecond);

    start + duration
}
