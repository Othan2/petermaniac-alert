mod bot;
mod client;
mod error;
use crate::groupme::client::GroupmeClient;
use std::rc::Rc;
pub use crate::twitter::get_tweet_urls_since;
use chrono::{Duration};


use bot::Bot;

pub fn start_groupme_bot(group_id: String, token: String, bot_id: String) {
    println!("starting groupme bot. group_id: {} token: {} bot_id: {}", group_id, token, bot_id);

    let bot: Bot = Bot {
        bot_id: bot_id.to_string(),
        client: Rc::new(GroupmeClient::new()),
    };

    loop {
        let res: Vec<String> =
            get_tweet_urls_since(chrono::Local::now().naive_local() - Duration::days(3)).unwrap();
        for tweet_url in res {
            bot.post(&tweet_url);
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
