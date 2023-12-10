use crate::{client, response};
use clap::{Arg, ArgMatches, Command};
use reqwest::StatusCode;
use crate::response::{handle_error, Response};

pub const COMMAND: &str = "zone";
const PATH: &str = "zones";

pub fn command_definition() -> Command {
    Command::new("zone")
        .about("Zone API Interface: ")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .about("List zones")
                .about("📝: https://developers.cloudflare.com/api/operations/zones-get")
                .arg(
                    Arg::new("domains")
                        .long("domains")
                        .short('d')
                        .num_args(0)
                        .help("list domains")
                )
        )
}

pub async fn call(sub_matches: &ArgMatches) {
    let command = sub_matches.subcommand().unwrap();
    match command {
        ("list", arg_matches) => {
            let response = client::get(PATH).await;
            match response.status() {
                StatusCode::OK => match arg_matches.contains_id("domains") {
                    true => domains_handler(response).await,
                    false => response::handle_default_ok(response).await
                },
                _ => handle_error(response)
            }
        }
        (name, _) => {
            unreachable!("Unsupported subcommand {}", name)
        }
    }
}

async fn domains_handler(response: reqwest::Response) {
    match response.json::<Response>().await {
        Ok(api_response) => {
            api_response.result.as_array().unwrap().iter()
                .for_each(|zone| println!("{}", zone["name"].as_str().unwrap()))
        },
        Err(e) => println!("ERROR: {:?}", e),
    };
}