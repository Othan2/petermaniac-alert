mod groupme;
mod twitter;
use chrono::NaiveDate;
pub use groupme::start_groupme_bot;
pub use twitter::{get_tweet_urls_since, get_twitter_result, TwitterResults};

use std::env;
use clap::Parser;
use log::Level;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Args {
    /// The groupme api token
   #[arg(short, long)]
    token: String,
    /// The id of the group for the bot
   #[arg(short, long)]
    group_id: String,
    /// The id of the bot
    #[arg(short, long)]
    bot_id: String,
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    // let res = run(query, None, None, None);
    // res.await;
    // await res;
    // println!("{:?}", res.wait());
    // let results: TwitterResults = getTweets(query, None, None, None).unwrap();
    start_groupme_bot(
        args.group_id,
        args.token,
        args.bot_id,
    );
}
