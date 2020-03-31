use std::collections::HashMap;
use crate::url::{Domain, MetaData};
use crate::app::load_domains;

#[derive(Clone)]
pub struct StaticMap {
    pub endpoints: HashMap<String,MetaData>,
}

impl StaticMap {

    pub fn from_file(path: &str) -> StaticMap {
        StaticMap::from_domains(load_domains(path))
    }

    fn from_domains(domains: Vec<Domain>) -> StaticMap {
        let mut map: HashMap<String,MetaData> = HashMap::new();

        for domain in domains {
            for endpoint in domain.endpoints {
                let key = format!("{} {}", endpoint.method, &endpoint.path);
                map.insert(key, MetaData::new(
                    domain.origin.clone(),
                    endpoint.groups,
                ));
            }
        }

        StaticMap {
            endpoints: map,
        }
    }

    pub fn get(&self, target: &str) -> Option<&MetaData> {
        self.endpoints.get(&target.to_string())
    }

}