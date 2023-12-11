use reqwest::StatusCode;
use crate::{client, response};

const PATH: &str = "user/tokens/verify";

pub async fn verify() {
    let response = client::get(PATH).await;
    match response.status() {
        StatusCode::OK => response::print(response).await,
        _ => response::handle_error(response)
    }
}
