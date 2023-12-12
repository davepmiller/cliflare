use crate::cli::{Cli, Commands, TokenCommands, ZoneCommands, ZoneDnsCommands};
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
            TokenCommands::Verify => token::verify().await,
        },
        Commands::Zone(zone) => match zone.command.unwrap() {
            ZoneCommands::List(args) => match args.domains {
                true => zone::list_domains().await,
                false => zone::list().await,
            },
            ZoneCommands::Create { domain } => zone::create(domain).await,
            ZoneCommands::Dns(dns) => match dns.command {
                ZoneDnsCommands::List { name, id } => {
                    let id = match name.is_none() {
                        true => id.unwrap(),
                        false => zone::get_id_from_name(name.unwrap()),
                    };
                    zone::dns_list(id).await
                }
            },
        },
    }
}
