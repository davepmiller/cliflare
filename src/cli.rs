use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    about = "Use Cloudflare API from the command line",
    author = "Dave Miller - https://github.com/davepmiller",
    long_about = None
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// List, Create, Zones
    Zone(ZoneArgs),
    /// Verify your API Token
    Token(TokenArgs),
    /// Manage DNS records
    Dns(DnsArgs),
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
    /// delete a zone
    #[command(arg_required_else_help = true)]
    Delete {
        /// zone name
        #[arg(long, short = 'n', required_unless_present = "zone_id")]
        zone_name: Option<String>,
        /// zone ID
        #[arg(long, short = 'i', required_unless_present = "zone_name")]
        zone_id: Option<String>,
    },
}

#[derive(Debug, Args)]
pub(crate) struct ZoneListArgs {
    /// List only domain names for each zone
    #[arg(short, long)]
    pub(crate) domains: bool,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct DnsArgs {
    #[command(subcommand)]
    pub(crate) command: DnsCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DnsCommands {
    /// List DNS records for a zone
    #[command(arg_required_else_help = true)]
    List {
        /// zone name
        #[arg(long, short = 'n', required_unless_present = "zone_id")]
        zone_name: Option<String>,
        /// zone ID
        #[arg(long, short = 'i', required_unless_present = "zone_name")]
        zone_id: Option<String>,
    },
    /// Export DNS records for a zone
    #[command(arg_required_else_help = true)]
    Export {
        /// zone name
        #[arg(long, short = 'n', required_unless_present = "zone_id")]
        zone_name: Option<String>,
        /// zone ID
        #[arg(long, short = 'i', required_unless_present = "zone_name")]
        zone_id: Option<String>,
    },
    /// Import DNS records to a zone
    #[command(arg_required_else_help = true)]
    Import {
        /// zone name
        #[arg(long, short = 'n', required_unless_present = "zone_id")]
        zone_name: Option<String>,
        /// zone ID
        #[arg(long, short = 'i', required_unless_present = "zone_name")]
        zone_id: Option<String>,
        /// BIND file to import
        #[arg(long, short, required = true)]
        file: String,
        /// Whether or not to proxy proxiable records
        #[arg(long, short)]
        proxy: bool,
    },
}
