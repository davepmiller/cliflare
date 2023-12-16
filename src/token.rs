use crate::client::{CloudflareClient, ENDPOINT};

const PATH: &str = "user/tokens/verify";

pub(crate) struct Token;

impl Token {
    pub(crate) fn verify() {
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(PATH);
        println!("{response:?}");
    }
}
