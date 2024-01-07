use crate::client;
use crate::client::RequestBody;
use crate::response::Response;

const PATH: &str = "zones";

pub(crate) struct Dns;

impl Dns {
    pub(crate) fn clear(id: &str) {
        let response = Dns::get_records(id);
        if response.result.as_array().is_none() {
            println!("result array is empty, make sure that DNS records exist for {id}");
            return;
        }
        response.result.as_array().unwrap().iter().for_each(|r| {
            Dns::delete_record(id, r["id"].as_str().unwrap());
        });
    }

    pub(crate) fn export(id: &str) {
        let path = format!(
            "{}/{}/dns_records/export?per_page=100",
            PATH,
            id.replace('\"', "")
        );
        let response = client::get(path.as_str());
        println!("{}", response.text.unwrap().as_str());
    }

    pub(crate) fn get_id(zone_name: Option<String>, zone_id: Option<String>) -> String {
        let matched_id = if zone_name.is_none() {
            zone_id.unwrap()
        } else {
            let name_arg = zone_name.unwrap().clone();
            let response = client::get(format!("{PATH}/?per_page=100").as_str());
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
        let response = client::post_form(path.as_str(), body);
        println!("{response:?}");
    }

    pub(crate) fn list(id: &str) {
        let response = Dns::get_records(id);
        if response.result.as_array().is_none() {
            println!("result array is empty, make sure that DNS records exist for {id}");
            return;
        }
        println!("{response:?}");
    }

    fn get_records(id: &str) -> Response {
        let path = format!("{}/{}/dns_records?per_page=100", PATH, id.replace('\"', ""));
        client::get(path.as_str())
    }

    fn delete_record(zone_id: &str, record_id: &str) -> Response {
        println!("deleting record: {record_id} for zone: {zone_id}");
        let path = format!(
            "{}/{}/dns_records/{}",
            PATH,
            zone_id.replace('\"', ""),
            record_id.replace('\"', "")
        );
        client::delete(path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::Dns;

    #[test]
    fn delete_record() {
        let res = Dns::delete_record("1234", "");
        assert_eq!(res.success, false);
    }
}
