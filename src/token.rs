use crate::{client, response};
use clap::{ArgMatches, Command};

pub const COMMAND: &str = "token";
const PATH: &str = "user/tokens/verify";

pub fn get_command() -> Command {
    Command::new(COMMAND)
        .about("User API Tokens Interface")
        .subcommand_required(true)
        .subcommand(
            Command::new("verify")
                .about("Sanity check your API token")
                .about("📝: https://developers.cloudflare.com/api/operations/zones-get"),
        )
}

pub async fn call(sub_matches: &ArgMatches) {
    let tokens_command = sub_matches.subcommand().unwrap();
    match tokens_command {
        ("verify", _) => {
            let response = client::get(PATH).await;
            if response.status() == reqwest::StatusCode::OK {
                response::handle_default_ok(response).await;
            } else {
                response::handle_error(response);
            }
        }
        (name, _) => {
            unreachable!("Unsupported subcommand {}", name)
        }
    }
}
