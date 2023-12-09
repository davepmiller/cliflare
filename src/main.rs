mod cli;
mod user_tokens;
mod client;

#[tokio::main]
async fn main() {
    let matches = cli::command().get_matches();
    match matches.subcommand() {
        Some((user_tokens::CMD, sub_matches)) => {
            user_tokens::call(sub_matches).await;
        }
        _ => unreachable!(),
    }
}
