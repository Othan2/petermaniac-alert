/*
Want to:
- poll for tweets periodically
- receive incoming petermaniac subscriptions
- send tweets if something meets criteria
     */
// extern crate groupme_bot;

// use groupme_bot::Groupme;
// extern crate twitter;
mod bot;
mod groupme;
mod client;
mod error;
mod bot_builder;
pub use crate::twitter::get_tweet_urls_since;
use chrono::NaiveDate;

use bot::Bot;
use groupme::Groupme;

pub fn start_groupme_bot(group_id: String, token: String) {
    println!("starting groupme bot. id: {} token: {}", group_id, token);

    let gm: Groupme = Groupme::new(Some(&token));
    let bot: Bot = gm
        .create_bot("Petermaniac bot", &group_id)
        .unwrap()
        .create()
        .unwrap();


    let res2: Vec<String> =
        get_tweet_urls_since(NaiveDate::from_ymd_opt(2023, 02, 20).unwrap()).unwrap();
    println!("{:?}", res2);
    println!("{:?}", bot.post(&res2[0]));
}

// pub fn poll_for_tweets
