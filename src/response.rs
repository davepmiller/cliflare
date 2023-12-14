use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub(crate) result: Value,
    pub(crate) result_info: Option<Value>,
    pub(crate) success: bool,
    pub(crate) errors: Vec<Value>,
    pub(crate) messages: Vec<Value>,
}
