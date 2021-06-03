use std::collections::hash_map::HashMap;
use serde_json::Value;

pub fn fetch_stats () -> HashMap<&'static str, i64> {
    let url = crate::get_env_var("ENDPOINT");
    let resp_text = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();

    let json: Value = serde_json::from_str(&resp_text[..])
        .expect("API didn't send JSON");
    let mut hash_map = HashMap::new();

    for field in crate::INTERP_FIELDS {
        let value = json["stats"][field]
            .as_i64()
            .expect("Invalid stat value");
        hash_map.insert(*field, value);
    }

    hash_map
}
