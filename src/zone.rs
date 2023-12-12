use crate::client::CloudflareClient;

const PATH: &str = "zones";

pub async fn list() {
    let response = CloudflareClient::get(PATH).await;
    println!("{:?}", response);
}

pub async fn list_domains() {
    let response = CloudflareClient::get(PATH).await;
    response
        .result
        .as_array()
        .unwrap()
        .iter()
        .for_each(|zone| println!("{}", zone["name"].as_str().unwrap()))
}

pub(crate) async fn create(name: String) {
    let response = CloudflareClient::post(PATH, name.to_string()).await;
    println!("{:?}", response);
}

pub(crate) async fn dns_list(id: String) {
    let path = format!("{}/{}/dns_records", PATH, id);
    let response = CloudflareClient::get(path.as_str()).await;
    println!("{:?}", response);
}
