use clap::{ArgMatches, Command};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::env;

pub const CMD: &str = "tokens";
const ENDPOINT: &str = "https://api.cloudflare.com/client/v4/user/tokens/verify";

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    errors: Vec<JSON>,
    messages: Vec<JSON>,
    result: JSON,
    success: bool,
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
    let token = match env::var("CLOUDFLARE_TOKEN") {
        Ok(t) => t,
        Err(_) => panic!("CLOUDFLARE_TOKEN is not set"),
    };
    let tokens_command = sub_matches.subcommand().unwrap();
    match tokens_command {
        ("verify", _) => {
            let client = reqwest::Client::new();
            let response = client
                .get(ENDPOINT)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap();
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<ApiResponse>().await {
                        Ok(parsed) => println!("Success! {:?}", parsed),
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
