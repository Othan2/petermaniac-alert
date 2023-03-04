mod groupme;
mod twitter;
use chrono::NaiveDate;
pub use groupme::start_groupme_bot;
pub use twitter::{get_tweet_urls_since, get_twitter_result, TwitterResults};

use std::env;

fn main() {
    // let res = run(query, None, None, None);
    // res.await;
    // await res;
    // println!("{:?}", res.wait());
    // let results: TwitterResults = getTweets(query, None, None, None).unwrap();
    start_groupme_bot(
        env::var("GROUPME_GROUP").unwrap(),
        env::var("GROUPME_TOKEN").unwrap(),
    );
}
