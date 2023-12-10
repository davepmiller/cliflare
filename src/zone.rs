use crate::{client, response};
use clap::{ArgMatches, Command};

pub const COMMAND: &str = "zones";
const PATH: &str = COMMAND;

pub fn command_definition() -> Command {
    Command::new("zone")
        .about("Zone API Interface: ")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .about("List zones")
                .about("📝: https://developers.cloudflare.com/api/operations/zones-get"),
        )
}

pub async fn call(sub_matches: &ArgMatches) {
    let command = sub_matches.subcommand().unwrap();
    match command {
        ("list", _) => {
            let response = client::get(PATH).await;
            response::handle(response).await;
        }
        (name, _) => {
            unreachable!("Unsupported subcommand {}", name)
        }
    }
}
