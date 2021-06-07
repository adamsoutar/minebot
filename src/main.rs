use std::{env, thread, time};
use discord::model::{
    ChannelId, MessageId, Game, GameType, OnlineStatus
};
use discord::Discord;
use chrono::prelude::*;
use crate::formatting::{n, d};

mod stats;
mod interp;
mod formatting;

// Bot will interpolate and edit the message every $this seconds
static UPDATE_PERIOD: u64 = 1000;
// This is the period we interpolate between
static REFETCH_PERIOD: u64 = 300000;
// We don't use all fields of the response
static INTERP_FIELDS: &[&str] = &[
    "gamesPlayed", "cubesSolved", "cubesExploded",
    "playTimeSeconds", "playerCount"
];

fn get_env_var(name: &str) -> String {
    env::var(name)
        .expect(&format!("Missing {} env var", name)[..])
}

fn main() {
    let token = get_env_var("BOT_TOKEN");
    let discord = Discord::from_bot_token(&token)
		.expect("Failed to log into Discord");

    let (connection, _) = discord.connect()
        .expect("Failed to connect to Discord");

    // Show our login in the client
    connection.set_presence(Some(Game {
        name: "Mastermine (obviously 😛)".to_string(),
        kind: GameType::Playing,
        url: Some("https://mastermine.app/".to_string())
    }), OnlineStatus::Online, false);

    let channel_num: u64 = get_env_var("CHANNEL_ID").parse().unwrap();
    let message_num: u64 = get_env_var("MESSAGE_ID").parse().unwrap();

    let channel_id = ChannelId(channel_num);
    let message_id = MessageId(message_num);

    let update_time = time::Duration::from_millis(UPDATE_PERIOD);
    let mut refetch_timer = 0;

    let mut fetched = stats::fetch_stats()
        .unwrap_or(stats::blank_stats());
    let mut fetched_next = fetched.clone();
    let mut fetch_timestamp = Utc::now();

    loop {
        refetch_timer += UPDATE_PERIOD;

        if refetch_timer >= REFETCH_PERIOD {
            refetch_timer = 0;

            fetched = fetched_next.clone();
            fetched_next = stats::fetch_stats()
                .unwrap_or(fetched.clone());
            fetch_timestamp = Utc::now();
        }

        let values = interp::interp_stats(&fetched, &fetched_next, fetch_timestamp);

        let msg_content = format!("
<:logo:787035070866784256> **Cubes attempted:** `{}`
<:explodedMine:787032733175644201> **Cubes exploded:** `{}`
<:tileFlagged:787032733280370769> **Cubes solved:** `{}`
:raising_hand: **Players:** `{}`
:stopwatch: **Combined time played:** {}",
        n(values["gamesPlayed"]), n(values["cubesExploded"]), n(values["cubesSolved"]),
        n(values["playerCount"]), d(values["playTimeSeconds"]));

        discord.edit_message(channel_id, message_id, &msg_content).unwrap();

        thread::sleep(update_time);
    }
}
