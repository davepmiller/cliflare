use crate::client;

const PATH: &str = "user/tokens/verify";

pub(crate) struct Token;

impl Token {
    pub(crate) fn verify() {
        let response = client::get(PATH);
        println!("{response:?}");
    }
}
