use crate::{client, response};
use crate::response::Response;

const PATH: &str = "zones";

pub async fn list() {
    let response = client::get(PATH).await;
    response::print(response).await
}

pub async fn list_domains() {
    let response = client::get(PATH).await;
    print_domains(response).await
}

async fn print_domains(response: reqwest::Response) {
    match response.json::<Response>().await {
        Ok(api_response) => {
            api_response.result.as_array().unwrap().iter()
                .for_each(|zone| println!("{}", zone["name"].as_str().unwrap()))
        },
        Err(e) => println!("ERROR: {:?}", e),
    };
}

pub(crate) async fn create(name: String) {
    let response = client::post(PATH, name.to_string()).await;
    response::print(response).await;
}