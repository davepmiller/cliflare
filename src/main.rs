use crate::cli::{Cli, Commands, DnsCommands, SettingsCommands, TokenCommands, ZoneCommands};
use crate::dns::Dns;
use crate::settings::Settings;
use crate::token::Token;
use crate::zone::Zone;
use clap::Parser;

mod cli;
mod client;
mod dns;
mod response;
mod settings;
mod token;
mod zone;

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Token(token) => match token.command {
            TokenCommands::Verify => Token::verify(),
        },
        Commands::Zone(zone) => match zone.command.unwrap() {
            ZoneCommands::List { domains } => {
                if domains {
                    Zone::list_domains();
                } else {
                    Zone::list();
                }
            }
            ZoneCommands::Create { domain } => Zone::create(domain),
            ZoneCommands::Delete { zone_id, zone_name } => {
                let id = Dns::get_id(zone_name, zone_id);
                Zone::delete(id.as_str());
            }
        },
        Commands::Dns(dns) => match dns.command {
            DnsCommands::List { zone_name, zone_id } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::list(id.as_str());
            }
            DnsCommands::Export { zone_name, zone_id } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::export(id.as_str());
            }
            DnsCommands::Import {
                zone_name,
                zone_id,
                file,
                proxy,
            } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::import(id.as_str(), file, proxy);
            }
            DnsCommands::Clear { zone_id, zone_name } => {
                let id = Dns::get_id(zone_name, zone_id);
                Dns::clear(id.as_str());
            }
        },
        Commands::Settings(settings) => match settings.command {
            SettingsCommands::List { zone_name, zone_id } => {
                let id = Dns::get_id(zone_name, zone_id);
                Settings::list(id.as_str());
            }
        },
    }
}
