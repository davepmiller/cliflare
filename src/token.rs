use crate::client::CloudflareClient;

const PATH: &str = "user/tokens/verify";

pub(crate) struct Token;

impl Token {
    pub(crate) fn verify() {
        let response = CloudflareClient::get(PATH);
        println!("{:?}", response);
    }
}
