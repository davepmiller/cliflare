use crate::client::{CloudflareClient, RequestBody, ENDPOINT};

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
        let mut body = RequestBody::default();
        body.name = Option::from(name);
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .post_json(PATH, body);
        println!("{:?}", response);
    }

    pub(crate) fn delete(id: String) {
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .delete(format!("{}/{}", PATH, id.as_str().replace('\"', "")));
        println!("{:?}", response);
    }
}
