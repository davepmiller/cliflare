use crate::client::CloudflareClient;

const PATH: &str = "zones";

pub(crate) struct Zone;

impl Zone {
    pub(crate) async fn list() {
        let response = CloudflareClient::get(PATH).await;
        println!("{:?}", response);
    }

    pub(crate) async fn list_domains() {
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
        let path = format!("{}/{}/dns_records", PATH, id.as_str().replace("\"", ""));
        let response = CloudflareClient::get(path.as_str()).await;
        println!("{:?}", response);
    }

    pub(crate) async fn get_id(name: Option<String>, id: Option<String>) -> String {
        let matched_id = match name.is_none() {
            true => id.unwrap(),
            false => {
                let name_arg = name.unwrap().clone();
                let response = CloudflareClient::get(PATH).await;
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
}
