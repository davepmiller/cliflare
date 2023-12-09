use clap::Command;
use crate::user_tokens;

pub(crate) fn command() -> Command {
    Command::new("cloudflare")
        .about("Cloudflare command line interface, written in Rust 🦀")
        .flatten_help(true)
        .subcommand_required(true)
        .subcommand(user_tokens::get_command())
}
