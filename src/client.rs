use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

const ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

pub async fn get(path: &str) -> reqwest::Response {
    let header = get_header();
    reqwest::Client::new()
        .get(format!("{}/{}", ENDPOINT, path))
        .header(AUTHORIZATION, header.auth)
        .header(CONTENT_TYPE, header.content_type)
        .send()
        .await
        .unwrap()
}

pub(crate) async fn post(path: &str, name: String) -> reqwest::Response {
    let header = get_header();
    let body = CreateBody {
        account: Account {
            id: get_account_id().to_string(),
        },
        name,
    };
    reqwest::Client::new()
        .post(format!("{}/{}", ENDPOINT, path))
        .header(AUTHORIZATION, header.auth)
        .json(&body)
        .send()
        .await
        .unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateBody {
    account: Account,
    name: String,
}

struct Header {
    auth: String,
    content_type: String,
}

fn get_header() -> Header {
    Header {
        auth: format!("Bearer {}", get_token()),
        content_type: "application/json".to_string(),
    }
}

fn get_token() -> String {
    match env::var("CLOUDFLARE_TOKEN") {
        Ok(t) => t,
        Err(_) => panic!("CLOUDFLARE_TOKEN is not set"),
    }
}

fn get_account_id() -> String {
    match env::var("CLOUDFLARE_ACCOUNT_ID") {
        Ok(id) => id.to_string(),
        Err(_) => panic!("CLOUDFLARE_ACCOUNT_ID is not set"),
    }
}
