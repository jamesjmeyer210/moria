use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::model::{AuthObj, Config};

#[derive(Serialize, Deserialize)]
struct Endpoint {
    path: String,
    method: String,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Domain {
    origin: String,
    endpoints: Vec<Endpoint>,
}

macro_rules! load_json_file {
    ($path:expr) => (
        serde_json::from_str(
            fs::read_to_string($path).unwrap_or_else(|error|{
                panic!("Error reading in file {}\n{}", $path, error)
            })
            .as_str()
        ).unwrap_or_else(|error|{
            panic!("Error converting file {}\n{}", $path, error);
        })
    )
}

pub fn load_config(path: &str) -> Config {
    load_json_file!(path)
}

pub fn load_endpoints(path: &str) -> HashMap<String,AuthObj> {
    let domains: Vec<Domain> = load_json_file!(path);

    let mut map: HashMap<String,AuthObj> = HashMap::new();

    for domain in domains {
        for endpoint in domain.endpoints {
            let key = format!("{} {}", endpoint.method, &endpoint.path);
            map.insert(key, AuthObj::new(
                domain.origin.clone(),
                endpoint.groups,
            ));
        }
    }

    return map;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[should_panic]
    fn load_config_fails_on_invalid_path(){
        load_config("/dev/null/config.json");
    }

    #[test]
    #[should_panic]
    fn load_config_fails_on_invalid_json(){
        load_config("test/test.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_path(){
        load_endpoints("/dev/null/endpoints.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_json(){
        load_config("test/test.json");
    }
}