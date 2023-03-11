use super::constants::{PETERMANIAC_SLUG, TWITTER_BASE_URL};
use super::headers::get_headers;
use super::models::{Tweet, User};
use super::parser::{get_next_cursor, get_tweets, get_users};
use super::request_builder::{build_request_config, RequestConfig};
use chrono::{NaiveDate, NaiveDateTime};
use postgres;
use reqwest::blocking::Response;
use serde_json::Value;
use std::collections::HashSet;
use tokio_postgres;
extern crate lazy_static;
use lazy_static::lazy_static;
use log::info;
use std::sync::Mutex;

pub struct TwitterClient {
    api_token: String,
    db_client: postgres::Client,
}

#[derive(Debug)]
struct TwitterResults {
    pub cursor: String,
    pub guest_token: String,
    pub tweets: Option<Vec<Tweet>>,
    pub users: Option<Vec<User>>,
}

impl TwitterClient {
    pub fn new(api_token: &str, db_client: postgres::Client) -> Self {
        Self {
            api_token: api_token.to_string(),
            db_client: db_client,
        }
    }

    fn get_twitter_result(
        &self,
        query: String,
        auth_token_option: Option<&'static str>,
        guest_token_option: Option<&'static str>,
        cursor: Option<String>,
    ) -> Result<TwitterResults, Box<dyn std::error::Error>> {
        info!("looking for tweets...");
        let headers_tuples: [(&'static str, &'static str); 2] =
            get_headers(&self.api_token, auth_token_option, guest_token_option).unwrap();
        let request_config: RequestConfig =
            build_request_config(&headers_tuples, query, cursor.clone());
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
        &mut self,
        since_date: NaiveDateTime,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let query: String = format!(
            "{} since:{}",
            PETERMANIAC_SLUG,
            since_date.format("%Y-%m-%d").to_string()
        );
        let results: TwitterResults = self.get_twitter_result(query, None, None, None).unwrap();

        let mut tweet_urls = Vec::new();

        for (tweet, user) in results
            .tweets
            .unwrap()
            .iter()
            .zip(results.users.unwrap().iter())
        {
            if tweet.favorite_count < 100 || !tweet.text.to_lowercase().contains(PETERMANIAC_SLUG) {
                continue;
            }

            let res = self.db_client.execute(
                "INSERT INTO tweet_ids(tweet_id) VALUES($1) ON CONFLICT DO NOTHING",
                &[&tweet.id.to_string()],
            )?;

            if res == 0 {
                tweet_urls.push(format!(
                    "https://twitter.com/{}/status/{}",
                    user.screen_name, tweet.id
                ));
            }
        }

        Ok(tweet_urls)
    }
}
