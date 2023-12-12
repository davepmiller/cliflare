use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    code: i32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResultInfo {
    count: i8,
    page: i8,
    per_page: i8,
    total_count: i8,
    total_pages: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub(crate) result: Value,
    pub(crate) result_info: Option<ResultInfo>,
    pub(crate) success: bool,
    pub(crate) errors: Vec<Value>,
    pub(crate) messages: Vec<Message>,
}
