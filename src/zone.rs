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

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::zone::{Zone, PATH};
    use mockito::Mock;
    use serde_json::json;

    fn list_mock() -> Mock {
        client::tests::build_server_mock("GET", format!("/{PATH}/?per_page=100").as_str())
            .with_body(
                json!({
                    "success": true,
                    "result": [],
                    "errors": [],
                    "messages": [],
                    "result_info": ""
                })
                .to_string(),
            )
            .create()
    }
    #[test]
    fn list() {
        let mock = list_mock();
        Zone::list();
        mock.assert();
    }

    #[test]
    fn list_domains() {
        let mock = list_mock();
        Zone::list_domains();
        mock.assert();
    }

    #[test]
    fn delete() {
        let id = "1234";
        let mock = client::tests::build_server_mock("DELETE", format!("/{}/{}", PATH, id).as_str())
            .with_body(client::tests::response_body())
            .create();
        Zone::delete(id);
        mock.assert();
    }

    #[test]
    fn create() {
        let id = "test.com";
        let mock = client::tests::build_server_mock("POST", format!("/{}", PATH).as_str())
            .with_body(client::tests::response_body())
            .create();
        Zone::create(id.to_string());
        mock.assert();
    }
}
