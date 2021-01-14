use std::collections::HashMap;
use crate::model::AuthObj;
use crate::auth::traits::Authentication;
use actix_http::http::Uri;
use std::rc::Rc;

pub struct UrlMap {
    internal: HashMap<Rc<Uri>, AuthObj>,
}

impl UrlMap {
    pub fn get(&self, url: &Uri) -> Option<&AuthObj> {
        self.internal.get(url)
    }
}

impl From<HashMap<Rc<Uri>, AuthObj>> for UrlMap {
    fn from(src: HashMap<Rc<Uri>, AuthObj>) -> Self {
        UrlMap {
            internal: src
        }
    }
}

impl Authentication<&Uri> for UrlMap {
    fn authenticate(&self, uri: &Uri) -> bool {
        self.internal.contains_key(uri)
    }
}