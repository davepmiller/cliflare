use crate::cli::{Cli, Commands, DnsCommands, TokenCommands, ZoneCommands};
use crate::dns::Dns;
use crate::token::Token;
use crate::zone::Zone;
use clap::Parser;

mod cli;
mod client;
mod dns;
mod response;
mod token;
mod zone;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::Token(token) => match token.command {
            TokenCommands::Verify => Token::verify(),
        },
        Commands::Zone(zone) => match zone.command.unwrap() {
            ZoneCommands::List(args) => match args.domains {
                true => Zone::list_domains(),
                false => Zone::list(),
            },
            ZoneCommands::Create { domain } => Zone::create(domain),
        },
        Commands::Dns(dns) => match dns.command {
            DnsCommands::List { zone_name, zone_id } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::list(id)
            }
            DnsCommands::Export { zone_name, zone_id } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::export(id);
            }
        },
    }

    Ok(())
}
