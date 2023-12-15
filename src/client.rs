use crate::response::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

pub const ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

pub(crate) struct CloudflareClient {
    pub(crate) endpoint: String,
}

impl CloudflareClient {
    pub(crate) fn get(&self, path: &str) -> Response {
        let headers = get_headers();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .send()
            .unwrap();
        parse_response(response)
    }

    pub(crate) fn post(&self, path: &str, body: Value) -> Response {
        let headers = get_headers();
        let account = json!({"account": {"id": get_env("CLOUDFLARE_ACCOUNT_ID")}});
        let req_body = vec![account, body];
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .json(&req_body)
            .send()
            .unwrap();
        parse_response(response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Account {
    pub(crate) id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestBody {
    pub(crate) account: Account,
    pub(crate) name: String,
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cloudflare_token = get_env("CLOUDFLARE_TOKEN");
    let auth = format!("Bearer {}", cloudflare_token);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_bytes(auth.as_bytes()).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

pub(crate) fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(t) => t,
        Err(_) => panic!("{} is not set", key),
    }
}

fn parse_response(response: reqwest::blocking::Response) -> Response {
    response.json::<Response>().unwrap_or_else(|e| {
        println!("{:?}", e);
        Response {
            result: Value::String("{}".to_string()),
            result_info: None,
            success: false,
            errors: vec![],
            messages: vec![],
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::env;

    #[test]
    fn test_get() {
        let mut mock_server = mockito::Server::new();
        let mock = mock_server
            .mock("GET", "/zones")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "success": true,
                    "result": "",
                    "errors": [],
                    "messages": [],
                    "result_info": ""
                })
                .to_string(),
            )
            .create();
        let client = CloudflareClient {
            endpoint: mock_server.url(),
        };
        let res = client.get("zones");

        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn test_post() {
        let mut mock_server = mockito::Server::new();
        let mock = mock_server
            .mock("POST", "/zones")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "success": true,
                    "result": "",
                    "errors": [],
                    "messages": [],
                    "result_info": ""
                })
                .to_string(),
            )
            .create();
        let client = CloudflareClient {
            endpoint: mock_server.url(),
        };
        let res = client.post("zones", json!({"name": "test.com"}));

        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn test_get_env() {
        let key = "TEST_KEY";
        let value = "TEST_VALUE";
        env::set_var(key, value);
        assert_eq!(get_env(key), value);
    }
}
