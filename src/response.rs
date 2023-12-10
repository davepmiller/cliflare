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
    result: Value,
    result_info: ResultInfo,
    success: bool,
    errors: Vec<Value>,
    messages: Vec<Message>,
}

pub async fn handle(response: reqwest::Response) {
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Response>().await {
                Ok(api_response) => println!("{:?}", api_response),
                Err(e) => println!("ERROR: {:?}", e),
            };
        }
        reqwest::StatusCode::BAD_REQUEST => {
            println!("400 UNAUTHORIZED");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("401 UNAUTHORIZED");
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("403: FORBIDDEN");
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    }
}
