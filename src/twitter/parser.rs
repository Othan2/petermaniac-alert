use crate::twitter::models::Tweet;
use crate::twitter::models::User;
use serde_json::Value;
use std::collections::HashMap;

type TweetsMap = HashMap<String, Tweet>;
type UsersMap = HashMap<String, User>;

pub fn get_tweets(body_data: &Value) -> Vec<Tweet> {
    let tweets_map: TweetsMap =
        serde_json::from_value(body_data["globalObjects"]["tweets"].clone()).unwrap();
    tweets_map.into_values().collect()
}
pub fn get_users(body_data: &Value) -> Vec<User> {
    let users_map: UsersMap =
        serde_json::from_value(body_data["globalObjects"]["users"].clone()).unwrap();
    users_map.into_values().collect()
}