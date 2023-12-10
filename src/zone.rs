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
        ("list", args) => {
            let response = client::get(PATH).await;
            handle_response(response, args).await
        }
        (name, _) => {
            unreachable!("Unsupported subcommand {}", name)
        }
    }
}

async fn handle_response(response: reqwest::Response, args: &ArgMatches) {
    match response.status() {
        StatusCode::OK => handle_ok(response, args).await,
        _ => handle_error(response)
    }
}

async fn handle_ok(response: reqwest::Response, args: &ArgMatches) {
    match args.contains_id("domains") {
        true => domains_handler(response).await,
        false => response::handle_default_ok(response).await
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