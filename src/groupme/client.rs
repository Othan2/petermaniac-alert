use crate::groupme::error::GroupmeError;

use reqwest::blocking::Client;
use std::collections::HashMap;
use http;

#[derive(Debug)]
pub struct GroupmeClient {
    path: String,
    client: Client
}

impl GroupmeClient {
    pub(super) fn new() -> GroupmeClient {
        GroupmeClient {
            path: "https://api.groupme.com/v3".to_string(),
            client: reqwest::blocking::Client::new()
        }
    }

    pub(super) fn post(
        &self,
        bot_id: &str,
        text: &str,
        picture_url: Option<&str>,
    ) -> Result<(), GroupmeError> {
        let mut body = HashMap::new();
        body.insert("bot_id", bot_id);
        body.insert("text", text);
        if let Some(picture_url) = picture_url {
            body.insert("picture_url", picture_url);
        }

        let url: String = format!("{}/bots/post", self.path);

        println!("{}", url.to_string());
        println!("{:#?}", &body);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()?;

        println!("posted\n{:?}", response);

        match response.status() {
            http::StatusCode::ACCEPTED => Ok(()),
            http::StatusCode::NOT_FOUND => Err(GroupmeError::AuthError),
            _ => Err(GroupmeError::BadHeaderError(response.status()))
        }
    }
}
