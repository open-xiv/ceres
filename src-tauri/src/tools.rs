use chrono::{Duration, TimeZone, Utc};

// time related
pub fn get_time(timestamp: i64, offset: i64) -> String {
    let utc_datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    let offest = Duration::hours(offset);
    let time = utc_datetime + offest;
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}
