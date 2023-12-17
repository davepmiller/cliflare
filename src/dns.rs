
use crate::client::{CloudflareClient, RequestBody, ENDPOINT};

const PATH: &str = "zones";

pub(crate) struct Dns;

impl Dns {
    pub(crate) fn export(id: &str) {
        let path = format!(
            "{}/{}/dns_records/export?per_page=100",
            PATH,
            id.replace('\"', "")
        );
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(path.as_str());
        println!("{}", response.text.unwrap().as_str());
    }

    pub(crate) fn get_id(zone_name: Option<String>, zone_id: Option<String>) -> String {
        let matched_id = if zone_name.is_none() {
            zone_id.unwrap()
        } else {
            let name_arg = zone_name.unwrap().clone();
            let response = CloudflareClient {
                endpoint: ENDPOINT.to_string(),
            }
            .get(format!("{PATH}/?per_page=100").as_str());
            match response
                .result
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["name"] == name_arg)
            {
                Some(v) => v["id"].to_string(),
                _ => panic!("No zone matching name: {name_arg}"),
            }
        };

        matched_id
    }

    pub(crate) fn import(id: &str, file: String, proxy: bool) {
        let path = format!("{}/{}/dns_records/import", PATH, id.replace('\"', ""));

        // let file_contents = fs::read_to_string(file).unwrap();
        let body = RequestBody {
            file: Option::from(file),
            proxied: Option::from(proxy.to_string()),
            ..Default::default()
        };
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .post_form(path.as_str(), body);
        println!("{response:?}");
    }

    pub(crate) fn list(id: &str) {
        let path = format!("{}/{}/dns_records?per_page=100", PATH, id.replace('\"', ""));
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(path.as_str());
        println!("{response:?}");
    }
}
