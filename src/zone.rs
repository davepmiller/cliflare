use crate::client;
use crate::client::RequestBody;

const PATH: &str = "zones";

pub(crate) struct Zone;

impl Zone {
    pub(crate) fn list() {
        let response = client::get(format!("{PATH}/?per_page=100").as_str());
        println!("{response:?}");
    }

    pub(crate) fn list_domains() {
        let response = client::get(format!("{PATH}/?per_page=100").as_str());
        response
            .result
            .as_array()
            .unwrap()
            .iter()
            .for_each(|zone| println!("{}", zone["name"].as_str().unwrap()));
    }

    pub(crate) fn create(name: String) {
        let body = RequestBody {
            name: Option::from(name),
            ..Default::default()
        };
        let response = client::post_json(PATH, body);
        println!("{response:?}");
    }

    pub(crate) fn delete(id: &str) {
        let response = client::delete(format!("{}/{}", PATH, id.replace('\"', "")).as_str());
        println!("{response:?}");
    }
}
