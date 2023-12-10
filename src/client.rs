use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;

const ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

fn get_token() -> String {
    match env::var("CLOUDFLARE_TOKEN") {
        Ok(t) => t,
        Err(_) => panic!("CLOUDFLARE_TOKEN is not set"),
    }
}

pub async fn get(path: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", ENDPOINT, path);
    let auth = format!("Bearer {}", get_token());
    let content_type = "application/json";
    client
        .get(url)
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, content_type)
        .send()
        .await
        .unwrap()
}
