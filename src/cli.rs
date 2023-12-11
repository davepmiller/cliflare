use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "Cliflare", author = "@davepmiller")]
#[command(version = "1.0")]
#[command(about = "Use Cloudflare API from the command line", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Zone(ZoneArgs),
    Token(TokenArgs),
}

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub(crate) command: TokenCommands,
}

#[derive(Debug, Subcommand)]
pub enum TokenCommands {
    Verify,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct ZoneArgs {
    #[command(subcommand)]
    pub(crate) command: Option<ZoneCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ZoneCommands {
    /// list zones
    List(ZoneListArgs),

    /// create a zone
    #[command(arg_required_else_help = true)]
    Create {
        /// provide a name for the zone
        #[arg(short, long, required = true)]
        domain: String,
    },
    /// interact with DNS records for a zone
    #[command(arg_required_else_help = true)]
    DNS(ZoneDNSArgs),
}

#[derive(Debug, Args)]
pub(crate) struct ZoneListArgs {
    /// List only domain names for each zone
    #[arg(short, long)]
    pub(crate) domains: bool,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct ZoneDNSArgs {
    #[command(subcommand)]
    pub(crate) command: ZoneDNSCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ZoneDNSCommands {
    /// List DNS records for a zone
    List {
        /// zone name
        #[arg(long, required_unless_present = "id")]
        name: Option<String>,
        /// zone ID
        #[arg(long, required_unless_present = "name")]
        id: Option<String>,
    },
}
