mod cli;
mod client;
mod response;
mod token;
mod zone;

#[tokio::main]
async fn main() {
    let matches = cli::command().get_matches();
    match matches.subcommand() {
        Some((token::COMMAND, sub_matches)) => {
            token::call(sub_matches).await;
        }
        Some((zone::COMMAND, sub_matches)) => {
            zone::call(sub_matches).await;
        }
        _ => unreachable!(),
    }
}
