use crate::twitter::constants::{GENERATE_GUEST_TOKEN_URL};
use reqwest::blocking::Client;
use serde_json::Value;

pub fn get_headers(
    default_auth_token: &str,
    auth_token_option: Option<&'static str>,
    guest_token_option: Option<&'static str>,
) -> Result<[(&'static str, &'static str); 2], Box<dyn std::error::Error>> {
    // TODO: clean up.
    let auth_token = auth_token_option.unwrap_or(Box::leak(default_auth_token.to_string().into_boxed_str()));
    let guest_token: &'static str;
    if guest_token_option.is_some() {
        guest_token = guest_token_option.unwrap();
    } else {
        let client: Client = Client::new();
        let response = client
            .post(GENERATE_GUEST_TOKEN_URL)
            .header("authorization", auth_token)
            .send()
            .unwrap();
        let body_data: Value = response.json::<Value>().unwrap();
        let boxed_guest_token: Box<str> = body_data["guest_token"]
            .as_str()
            .unwrap()
            .to_string()
            .into_boxed_str();
        // TODO: clean up.
        // we have to force string from request body to be static str for headers map for the next request
        guest_token = Box::leak(boxed_guest_token);
    }
    let headers: [(&'static str, &'static str); 2] = [
        ("authorization", auth_token),
        ("x-guest-token", guest_token),
    ];
    Ok(headers)
}