mod constants;
mod headers;
mod models;
mod parser;
mod request_builder;
use chrono::{NaiveDate, NaiveDateTime};
use constants::TWITTER_BASE_URL;
use headers::get_headers;
pub use models::{Tweet, User};
use parser::{get_next_cursor, get_tweets, get_users};
use request_builder::RequestConfig;
use reqwest::blocking::{Client, Response};
use serde_json::Value;
use std::collections::HashSet;
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct TwitterResults {
    pub cursor: String,
    pub guest_token: String,
    pub tweets: Option<Vec<Tweet>>,
    pub users: Option<Vec<User>>,
}

pub fn get_twitter_result(
    query: String,
    auth_token_option: Option<&'static str>,
    guest_token_option: Option<&'static str>,
    cursor: Option<String>,
) -> Result<TwitterResults, Box<dyn std::error::Error>> {
    let headers_tuples: [(&'static str, &'static str); 2] =
        get_headers(auth_token_option, guest_token_option).unwrap();
    let request_config: RequestConfig =
        request_builder::build_request_config(&headers_tuples, query, cursor.clone());
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let response: Response = client
        .get(TWITTER_BASE_URL)
        .query(&request_config.path_query)
        .headers(request_config.headers)
        .send()
        .unwrap();
    let body_data: Value = response.json::<Value>()?;
    let next_cursor: String = get_next_cursor(&body_data, cursor)?;
    let tweets: Vec<Tweet> = get_tweets(&body_data);
    let users: Vec<User> = get_users(&body_data);
    let guest_token: String = headers_tuples[1].1.to_string();
    let twitter_results = TwitterResults {
        cursor: next_cursor,
        guest_token,
        tweets: Some(tweets),
        users: Some(users),
    };
    Ok(twitter_results)
}

pub fn get_tweet_urls_since(
    since_date: NaiveDateTime,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // For deduplicating tweets
    lazy_static! {
        static ref TWEET_IDS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
    }

    let query: String = format!(
        "{} since:{}",
        constants::PETERMANIAC_SLUG,
        since_date.format("%Y-%m-%d").to_string()
    );
    let results: TwitterResults = get_twitter_result(query, None, None, None).unwrap();

    let mut tweet_urls = Vec::new();

    for (tweet, user) in results
        .tweets
        .unwrap()
        .iter()
        .zip(results.users.unwrap().iter())
    {
        if (tweet.favorite_count > 100
            && tweet
                .text
                .to_lowercase()
                .contains(constants::PETERMANIAC_SLUG)
            && TWEET_IDS.lock().unwrap().insert(tweet.id))
        {
            tweet_urls.push(format!(
                "https://twitter.com/{}/status/{}",
                user.screen_name, tweet.id
            ));
        }
    }

    Ok(tweet_urls)
}
