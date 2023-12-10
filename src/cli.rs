use crate::{token, zone};
use clap::Command;

pub fn command() -> Command {
    Command::new("cliflare")
        .about("Cloudflare command line interface, written in Rust 🦀")
        .about("🌎: CLOUDFLARE_TOKEN environment variable is required")
        .flatten_help(true)
        .subcommand_required(true)
        .subcommand(token::get_command())
        .subcommand(zone::command_definition())
}
