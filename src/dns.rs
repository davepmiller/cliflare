use crate::client::{CloudflareClient, ENDPOINT};

const PATH: &str = "zones";

pub(crate) struct Dns;

impl Dns {
    pub(crate) fn export(id: String) {
        let path = format!(
            "{}/{}/dns_records/export?per_page=100",
            PATH,
            id.as_str().replace('\"', "")
        );
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(path.as_str());
        println!("{:?}", response);
    }

    pub(crate) fn get_id(zone_name: Option<String>, zone_id: Option<String>) -> String {
        let matched_id = match zone_name.is_none() {
            true => zone_id.unwrap(),
            false => {
                let name_arg = zone_name.unwrap().clone();
                let response = CloudflareClient {
                    endpoint: ENDPOINT.to_string(),
                }
                .get(format!("{}/?per_page=100", PATH).as_str());
                match response
                    .result
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|v| v["name"] == name_arg)
                {
                    Some(v) => v["id"].to_string(),
                    _ => panic!("No zone matching name: {}", name_arg),
                }
            }
        };

        matched_id
    }

    pub(crate) fn list(id: String) {
        let path = format!(
            "{}/{}/dns_records?per_page=100",
            PATH,
            id.as_str().replace('\"', "")
        );
        let response = CloudflareClient {
            endpoint: ENDPOINT.to_string(),
        }
        .get(path.as_str());
        println!("{:?}", response);
    }
}
