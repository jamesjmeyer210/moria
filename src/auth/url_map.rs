use std::collections::HashMap;
use crate::model::AuthObj;

pub struct UrlMap {
    internal: HashMap<String, AuthObj>,
}

impl UrlMap {
    fn get(&self, url: &str) -> Option<&AuthObj> {
        self.internal.get(url)
    }
}

impl From<HashMap<String, AuthObj>> for UrlMap {
    fn from(src: HashMap<String, AuthObj>) -> Self {
        UrlMap {
            internal: src
        }
    }
}