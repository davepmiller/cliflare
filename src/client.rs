use crate::response::Response;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

const ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

pub struct CloudflareClient;

impl CloudflareClient {
    pub async fn get(path: &str) -> Response {
        let header = get_header();
        let response = reqwest::Client::new()
            .get(format!("{}/{}", ENDPOINT, path))
            .header(AUTHORIZATION, header.auth)
            .header(CONTENT_TYPE, header.content_type)
            .send()
            .await
            .unwrap();
        let decoded_response = response.json::<Response>().await;
        match decoded_response {
            Ok(dr) => dr,
            Err(e) => {
                println!("{:?}", e);
                Response {
                    result: Value::String("{}".to_string()),
                    result_info: None,
                    success: false,
                    errors: vec![],
                    messages: vec![],
                }
            }
        }
    }

    pub(crate) async fn post(path: &str, name: String) -> reqwest::Response {
        let header = get_header();
        let body = CreateBody {
            account: Account {
                id: get_env("CLOUDFLARE_ACCOUNT_ID"),
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
        auth: format!("Bearer {}", get_env("CLOUDFLARE_TOKEN")),
        content_type: "application/json".to_string(),
    }
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(t) => t,
        Err(_) => panic!("{} is not set", key),
    }
}

#[test]
fn test_get_env() {
    let key = "TEST_KEY";
    let value = "TEST_VALUE";
    env::set_var(key, value);
    assert_eq!(get_env(key), value);
}
