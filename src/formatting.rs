use num_format::{Locale, ToFormattedString};

// Formats numbers
pub fn n(num: i64) -> String {
    num.to_formatted_string(&Locale::en)
}

// Formats durations
pub fn d(duration_secs: i64) -> String {
    let dur = chrono::Duration::seconds(duration_secs);
    let dur_hrs = dur.num_hours();
    let dur_mins = dur.num_minutes() - dur_hrs * 60;
    let dur_secs = dur.num_seconds() - dur.num_minutes() * 60;
    let dur_years = dur.num_days() / 365;
    format!("`{}h {}m {}s` _({} years! :scream:)_", n(dur_hrs), n(dur_mins), n(dur_secs), n(dur_years))
}
