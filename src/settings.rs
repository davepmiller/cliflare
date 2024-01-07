use crate::client;
use crate::response::Response;

pub(crate) struct Settings;

impl Settings {
    pub(crate) fn list(zone_id: &str) {
        let response = Settings::get_settings(zone_id);
        if response.result.as_array().is_none() {
            panic!("result array is empty, does the zone exist? zone_id:{zone_id}");
        }
        println!("{response:?}");
    }

    fn get_settings(zone_id: &str) -> Response {
        let path = format!("zones/{}/settings?per_page=100", zone_id.replace('\"', ""));
        client::get(path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::client::tests::response_body;
    use crate::settings::Settings;

    #[test]
    #[should_panic(expected = "result array is empty")]
    fn list() {
        let path = "/zones/1234/settings?per_page=100";
        let server_mock = client::tests::build_server_mock("GET", path);
        server_mock.with_body(response_body()).create();
        Settings::list("");
    }

    #[test]
    fn get_settings() {
        let fake_id = "1234";
        let path = format!("/zones/{fake_id}/settings?per_page=100");
        let server_mock = client::tests::build_server_mock("GET", path.as_str());
        let mock = server_mock.with_body(response_body()).create();

        let res = Settings::get_settings(fake_id);
        mock.assert();
        assert!(res.success);
    }
}
