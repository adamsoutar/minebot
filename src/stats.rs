use serde_json::Value;
use std::collections::hash_map::HashMap;

pub fn fetch_stats() -> reqwest::Result<HashMap<&'static str, i64>> {
    let url = crate::get_env_var("ENDPOINT");
    let resp_text = reqwest::blocking::get(url)?.text()?;

    println!("resp_text: \"{}\"", resp_text);

    let json: Value = serde_json::from_str(&resp_text[..])?;
    let mut hash_map = HashMap::new();

    for field in crate::INTERP_FIELDS {
        let value = json["stats"][field].as_i64().expect("Invalid stat value");
        hash_map.insert(*field, value);
    }

    Ok(hash_map)
}

pub fn blank_stats() -> HashMap<&'static str, i64> {
    let mut hash_map = HashMap::new();

    for field in crate::INTERP_FIELDS {
        hash_map.insert(*field, 0);
    }

    hash_map
}
