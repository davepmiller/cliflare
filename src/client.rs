use crate::response::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::env;

pub const ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

pub(crate) struct CloudflareClient {
    pub(crate) endpoint: String,
}

#[derive(Serialize, Default, Debug)]
pub(crate) struct Account {
    id: String,
}

#[derive(Serialize, Default, Debug)]
pub(crate) struct RequestBody {
    pub(crate) account: Option<Account>,
    pub(crate) name: Option<String>,
    pub(crate) file: Option<String>,
    pub(crate) proxied: Option<String>,
}

impl CloudflareClient {
    pub(crate) fn get(&self, path: &str) -> Response {
        let headers = build_headers();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .send()
            .unwrap();
        match path.contains("export") {
            true => parse_text(response),
            false => parse_response(response),
        }
    }

    pub(crate) fn post_json(&self, path: &str, mut body: RequestBody) -> Response {
        let headers = build_headers();
        body.account = Option::from(Account {
            id: get_env("CLOUDFLARE_ACCOUNT_ID"),
        });
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .json(&body)
            .send()
            .unwrap();
        parse_response(response)
    }

    pub(crate) fn post_form(&self, path: &str, body: RequestBody) -> Response {
        let mut headers = build_headers();
        headers.remove(CONTENT_TYPE);
        let form = reqwest::blocking::multipart::Form::new().file("file", body.file.unwrap());

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .multipart(form.unwrap())
            .send()
            .unwrap();
        parse_response(response)
    }

    pub(crate) fn delete(&self, path: String) -> Response {
        let headers = build_headers();
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(format!("{}/{}", self.endpoint, path))
            .headers(headers)
            .send()
            .unwrap();
        parse_response(response)
    }
}

fn parse_text(response: reqwest::blocking::Response) -> Response {
    Response {
        result: Value::String("{}".to_string()),
        result_info: None,
        success: false,
        errors: vec![],
        messages: vec![],
        text: Some(response.text().unwrap_or_else(|e| {
            println!("{:?}", e);
            "".to_string()
        })),
    }
}

fn build_headers() -> HeaderMap {
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

fn get_env(key: &str) -> String {
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
            text: Some("".to_string()),
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::env;

    fn response_body() -> String {
        json!({
        "success": true,
        "result": "",
        "errors": [],
        "messages": [],
        "result_info": ""
        })
        .to_string()
    }

    #[test]
    fn test_get() {
        let mut mock_server = mockito::Server::new();
        let mock = mock_server
            .mock("GET", "/zones")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(response_body())
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
                    "result_info": "",
                    "text": ""
                })
                .to_string(),
            )
            .create();
        let client = CloudflareClient {
            endpoint: mock_server.url(),
        };
        let mut body = RequestBody::default();
        body.name = Option::from("test.com".to_string());
        let res = client.post_json("zones", body);

        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn test_delete() {
        let mut mock_server = mockito::Server::new();
        let mock = mock_server
            .mock("DELETE", "/zones/1234")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "success": true,
                    "result": "",
                    "errors": [],
                    "messages": [],
                    "result_info": "",
                    "text": ""
                })
                .to_string(),
            )
            .create();
        let client = CloudflareClient {
            endpoint: mock_server.url(),
        };
        let mut body = RequestBody::default();
        body.name = Option::from("test.com".to_string());
        let res = client.delete("zones/1234".to_string());

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
