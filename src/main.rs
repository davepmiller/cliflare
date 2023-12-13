use crate::cli::{Cli, Commands, TokenCommands, ZoneCommands, ZoneDnsCommands};
use crate::token::Token;
use crate::zone::Zone;
use clap::Parser;

mod cli;
mod client;
mod response;
mod token;
mod zone;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Token(token) => match token.command {
            TokenCommands::Verify => Token::verify().await,
        },
        Commands::Zone(zone) => match zone.command.unwrap() {
            ZoneCommands::List(args) => match args.domains {
                true => Zone::list_domains().await,
                false => Zone::list().await,
            },
            ZoneCommands::Create { domain } => Zone::create(domain).await,
            ZoneCommands::Dns(dns) => match dns.command {
                ZoneDnsCommands::List { name, id } => {
                    let id = Zone::get_id(name, id).await;
                    Zone::dns_list(id).await
                }
            },
        },
    }
}
