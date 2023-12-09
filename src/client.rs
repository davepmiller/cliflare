use std::env;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

fn get_token() -> String {
    match env::var("CLOUDFLARE_TOKEN") {
        Ok(t) => t,
        Err(_) => panic!("CLOUDFLARE_TOKEN is not set"),
    }
}

pub(crate) async fn get(uri: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    let auth = format!("Bearer {}", get_token());
    let content_type = "application/json";
    client.get(uri)
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, content_type)
        .send()
        .await
        .unwrap()
}