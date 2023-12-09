use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::fmt::{Display, Formatter};
use crate::client;

pub const CMD: &str = "tokens";
const ENDPOINT: &str = "https://api.cloudflare.com/client/v4/user/tokens/verify";

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    id: String,
    status: String,
    not_before: String,
    expires_on: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    code: i32,
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    errors: Vec<JSON>,
    messages: Vec<Message>,
    result: Result,
    success: bool,
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            for message in &self.messages {
                return write!(f, "{}", message.message)
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseInfo {
    count: u32,
    page: u32,
    per_page: u32,
    total_count: u32,
    total_pages: u32,
}

pub(crate) fn get_command() -> Command {
    Command::new("tokens")
        .about("User API Tokens Interface")
        .subcommand_required(true)
        .subcommand(
            Command::new("verify")
                .about("Verify a token, requires CLOUDFLARE_TOKEN environment variable to be set"),
        )
}

pub(crate) async fn call(sub_matches: &ArgMatches) {
    let tokens_command = sub_matches.subcommand().unwrap();
    match tokens_command {
        ("verify", _) => {
            let response = client::get(ENDPOINT).await;
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<ApiResponse>().await {
                        Ok(api_response) => println!("{}", api_response),
                        Err(e) => println!("Something went wrong, response error {:?}", e),
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
        (name, _) => {
            unreachable!("Unsupported subcommand {}", name)
        }
    }
}
