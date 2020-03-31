use serde::{Serialize, Deserialize};

pub mod url_type;
pub mod static_map;
pub mod dynamic_map;

use static_map::StaticMap;
use dynamic_map::DynamicMap;

#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub origin: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Clone)]
pub struct MetaData {
    pub origin: String,
    pub groups: Vec<String>,
}

impl MetaData {
    pub fn new(o: String, g: Vec<String>) -> Self {
        MetaData {
            origin: o,
            groups: g,
        }
    }
}

pub struct UrlTree {
    fixed: StaticMap,
    dynamic: DynamicMap,
}

impl UrlTree {

    fn from_maps(s: StaticMap, u: DynamicMap) -> UrlTree {
        UrlTree {
            fixed: s,
            dynamic: u,
        }
    }

    pub fn try_fixed_get(&self, target: &str) -> Option<&MetaData> {
        self.fixed.get(target)
    }

    pub fn try_dynamic_get(&self, target: &str) -> Option<&MetaData> {
        self.dynamic.get(target)
    }

}