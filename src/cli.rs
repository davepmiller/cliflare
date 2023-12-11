use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "Cliflare")]
#[command(author = "@davepmiller")]
#[command(version = "1.0")]
#[command(about = "Use Cloudflare API from the command line", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Zone(ZoneArgs),
    Token(TokenArgs)
}

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub(crate) command: Option<TokenCommands>,
}

#[derive(Debug, Subcommand)]
pub enum TokenCommands {
    Verify
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct ZoneArgs {
    #[command(subcommand)]
    pub(crate) command: Option<ZoneCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ZoneCommands {
    List(ZoneListArgs),
    #[command(arg_required_else_help = true)]
    Create {
        domain: String
    }
}

#[derive(Debug, Args)]
pub struct ZoneListArgs {
    #[arg(short, long)]
    pub(crate) domains: bool
}
