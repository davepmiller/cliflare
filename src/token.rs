use crate::client::CloudflareClient;

const PATH: &str = "user/tokens/verify";

pub async fn verify() {
    let response = CloudflareClient::get(PATH).await;
    println!("{:?}", response);
}
