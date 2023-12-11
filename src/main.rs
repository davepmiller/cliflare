use clap::Parser;
use crate::cli::{Cli, Commands, TokenCommands, ZoneCommands};

mod cli;
mod client;
mod response;
mod token;
mod zone;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Token(token) => {
            let token_cmd = token.command.unwrap_or(TokenCommands::Verify);
            match token_cmd {
                TokenCommands::Verify => token::verify().await
            }
        }
        Commands::Zone(zone) => {
            match zone.command.unwrap() {
                ZoneCommands::List(args) => {
                    match args.domains {
                        true => zone::list_domains().await,
                        false => zone::list().await
                    }
                }
                ZoneCommands::Create{domain} => {
                    zone::create(domain).await
                }
            }
        }
    }
}
