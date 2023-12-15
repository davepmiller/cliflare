use crate::client::{CloudflareClient, ENDPOINT};
use serde_json::json;

const PATH: &str = "zones";

pub(crate) struct Zone;

impl Zone {
    pub(crate) fn list() {
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(format!("{}/?per_page=100", PATH).as_str());
        println!("{:?}", response);
    }

    pub(crate) fn list_domains() {
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(format!("{}/?per_page=100", PATH).as_str());
        response
            .result
            .as_array()
            .unwrap()
            .iter()
            .for_each(|zone| println!("{}", zone["name"].as_str().unwrap()))
    }

    pub(crate) fn create(name: String) {
        let body = json!({"name": name.as_str()});
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .post(PATH, body);
        println!("{:?}", response);
    }
}
