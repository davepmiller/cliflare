use crate::response::Response;
use crate::{client, response};

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
        Ok(api_response) => api_response
            .result
            .as_array()
            .unwrap()
            .iter()
            .for_each(|zone| println!("{}", zone["name"].as_str().unwrap())),
        Err(e) => println!("ERROR: {:?}", e),
    };
}

pub(crate) async fn create(name: String) {
    let response = client::post(PATH, name.to_string()).await;
    response::print(response).await;
}

pub(crate) async fn dns_list(id: String) {
    let path = format!("{}/{}/dns_records", PATH, id);
    let response = client::get(path.as_str()).await;
    response::print(response).await
}

pub(crate) fn get_id_from_name(name: String) -> String {
    format!("{}", name)
}
