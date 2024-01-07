use crate::response::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::env;

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

// impl CloudflareClient {
pub(crate) fn get(path: &str) -> Response {
    let headers = build_headers();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("{}/{}", get_env("CLOUDFLARE_ENDPOINT"), path))
        .headers(headers)
        .send()
        .unwrap();
    if path.contains("/dns_records/export") {
        parse_text(response)
    } else {
        parse_response(response)
    }
}

pub(crate) fn post_json(path: &str, mut body: RequestBody) -> Response {
    let headers = build_headers();
    body.account = Option::from(Account {
        id: get_env("CLOUDFLARE_ACCOUNT_ID"),
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/{}", get_env("CLOUDFLARE_ENDPOINT"), path))
        .headers(headers)
        .json(&body)
        .send()
        .unwrap();
    parse_response(response)
}

pub(crate) fn post_form(path: &str, body: RequestBody) -> Response {
    let mut headers = build_headers();
    headers.remove(CONTENT_TYPE);
    let form = reqwest::blocking::multipart::Form::new().file("file", body.file.unwrap());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/{}", get_env("CLOUDFLARE_ENDPOINT"), path))
        .headers(headers)
        .multipart(form.unwrap())
        .send()
        .unwrap();
    parse_response(response)
}

pub(crate) fn delete(path: &str) -> Response {
    let headers = build_headers();
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(format!("{}/{}", get_env("CLOUDFLARE_ENDPOINT"), path))
        .headers(headers)
        .send()
        .unwrap();
    parse_response(response)
}

fn parse_text(response: reqwest::blocking::Response) -> Response {
    Response {
        result: Value::String("{}".to_string()),
        result_info: None,
        success: false,
        errors: vec![],
        messages: vec![],
        text: Some(response.text().unwrap_or_else(|e| {
            println!("{e:?}");
            String::new()
        })),
    }
}

fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cloudflare_token = get_env("CLOUDFLARE_TOKEN");
    let auth = format!("Bearer {cloudflare_token}");
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_bytes(auth.as_bytes()).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

pub(crate) fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} should be set"))
}

fn parse_response(response: reqwest::blocking::Response) -> Response {
    response.json::<Response>().unwrap_or_else(|e| {
        println!("{e:?}");
        Response {
            result: Value::String("{}".to_string()),
            result_info: None,
            success: false,
            errors: vec![],
            messages: vec![],
            text: Some(String::new()),
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::client;
    use mockito;
    use mockito::Mock;
    use serde_json::json;
    use std::env;

    pub(crate) fn response_body() -> String {
        json!({
            "success": true,
            "result": "",
            "errors": [],
            "messages": [],
            "result_info": ""
        })
        .to_string()
    }

    pub(crate) fn build_server_mock(method: &str, path: &str) -> Mock {
        let mut mock_server = mockito::Server::new();
        env::set_var("CLOUDFLARE_ENDPOINT", mock_server.url().replace('\"', ""));
        env::set_var("CLOUDFLARE_TOKEN", "test1234");
        env::set_var("CLOUDFLARE_ACCOUNT_ID", "test1234");
        mock_server
            .mock(method, path)
            .with_status(201)
            .with_header("content-type", "application/json")
    }

    #[test]
    fn get() {
        let server_mock = build_server_mock("GET", "/zones");
        let mock = server_mock.with_body(response_body()).create();
        let res = client::get("zones");

        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn post_json() {
        let server_mock = build_server_mock("POST", "/zones");
        let mock = server_mock
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
        let mut body = RequestBody::default();
        body.name = Option::from("test.com".to_string());
        let res = client::post_json("zones", body);

        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn delete() {
        let server_mock = build_server_mock("DELETE", "/zones/1234");
        let mock = server_mock
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

        let res = client::delete("zones/1234");
        mock.assert();
        assert!(res.success);
    }

    #[test]
    fn get_env() {
        let key = "TEST_KEY";
        let value = "TEST_VALUE";
        env::set_var(key, value);
        assert_eq!(client::get_env(key), value);
        env::remove_var(key);
    }

    #[test]
    #[should_panic]
    fn get_env_not_set() {
        client::get_env("TEST_KEY");
    }
}
