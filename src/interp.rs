use chrono::prelude::*;
use std::collections::HashMap;

// Take two stats collections and find the values in the middle based on the time
pub fn interp_stats (
    prev: &HashMap<&str, i64>,
    next: &HashMap<&str, i64>,
    timestamp: DateTime::<Utc>
) -> HashMap<&'static str, i64> {
    let now = Utc::now();
    let prog = now
        .signed_duration_since(timestamp)
        .num_milliseconds() as f64 / crate::REFETCH_PERIOD as f64;

    let mut interped = HashMap::new();

    for field in crate::INTERP_FIELDS {
        let diff = next[*field] - prev[*field];
        let progged = diff as f64 * prog;
        let rounded = prev[*field] + progged as i64;
        interped.insert(*field, rounded);
    }

    interped
}
