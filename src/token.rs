use crate::client;

const PATH: &str = "user/tokens/verify";

pub(crate) struct Token;

impl Token {
    pub(crate) fn verify() {
        let response = client::get(PATH);
        println!("{response:?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::token::{Token, PATH};

    #[test]
    fn verify() {
        let mock = client::tests::build_server_mock("GET", format!("/{}", PATH).as_str())
            .with_body(client::tests::response_body())
            .create();
        Token::verify();
        mock.assert();
    }
}
