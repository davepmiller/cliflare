use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    code: i32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultInfo {
    count: i8,
    page: i8,
    per_page: i8,
    total_count: i8,
    total_pages: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub(crate) result: Value,
    result_info: ResultInfo,
    success: bool,
    errors: Vec<Value>,
    messages: Vec<Message>,
}

pub async fn handle_default_ok(response: reqwest::Response) {
    if response.status() != StatusCode::OK {
        panic!("handle_default_ok requires StatusCode::OK, was passed: {}", response.status())
    }

    match response.json::<Response>().await {
        Ok(api_response) => println!("{:?}", api_response),
        Err(e) => println!("ERROR: {:?}", e),
    };
}

pub(crate) fn handle_error(response: reqwest::Response) {
    match response.status() {
        StatusCode::BAD_REQUEST => {
            println!("400 UNAUTHORIZED");
        }
        StatusCode::UNAUTHORIZED => {
            println!("401 UNAUTHORIZED");
        }
        StatusCode::FORBIDDEN => {
            println!("403: FORBIDDEN");
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    }
}