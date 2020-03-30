use std::collections::HashMap;
use actix_web::http::Method;
use crate::url::metadata::MetaData;

struct StaticMap {
    endpoints: HashMap<String,MetaData>,
}

impl StaticMap {

    fn from(values: Vec<Domain>) -> StaticMap {
        let mut map: HashMap<String,MetaData> = HashMap::new();

        for domain in domains {
            for endpoint in domain.endpoints {
                let key = format!("{} {}", endpoint.method, &endpoint.path);
                map.insert(key, AuthObj::new(
                    domain.origin.clone(),
                    endpoint.groups,
                ));
            }
        }

        StaticMap {
            endpoints: map,
        }
    }

    fn get(&self, method: &Method, url: &str) -> Option<Metadat> {
        let lookup = format!("{} {}", req.method(), req.path());
        self.endpoints.get(&lookup)
    }

}