use crate::client::CloudflareClient;

const PATH: &'static str = "zones";

pub(crate) struct Zone;

impl Zone {
    pub(crate) async fn list() {
        let response = CloudflareClient::get(format!("{}/?per_page=100", PATH).as_str()).await;
        println!("{:?}", response);
    }

    pub(crate) async fn list_domains() {
        let response = CloudflareClient::get(format!("{}/?per_page=100", PATH).as_str()).await;
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
}
