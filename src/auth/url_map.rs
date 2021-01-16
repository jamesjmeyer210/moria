use std::collections::HashMap;
use crate::model::AuthObj;
use crate::auth::traits::Authentication;
use actix_http::http::Uri;
use std::rc::Rc;

pub struct UriMap {
    internal: HashMap<Uri, Vec<String>>,
}

impl UriMap {
    pub fn new() -> Self {
        UriMap {
            internal: HashMap::new(),
        }
    }

    pub fn get(&self, url: &Uri) -> Option<&Vec<String>> {
        self.internal.get(url)
    }
}

impl From<HashMap<Uri, Vec<String>>> for UriMap {
    fn from(src: HashMap<Uri, Vec<String>>) -> Self {
        UriMap {
            internal: src
        }
    }
}

#[cfg(test)]
mod test {
    use crate::auth::url_map::UriMap;
    use std::collections::HashMap;
    use actix_http::http::Uri;

    #[test]
    fn get_returns_some_when_uri_is_match() -> () {
        let mut hashmap = HashMap::with_capacity(1);
        hashmap.insert(Uri::from_static("https://abc.api")
                       , vec!["api_user".to_string(), "admin".to_string()]);

        let uri_map = UriMap::from(hashmap);

        let result = uri_map.get(&Uri::from_static("https://abc.api"));
        assert!(result.is_some());
        assert_eq!(&vec!["api_user".to_string(), "admin".to_string()], result.unwrap());
    }

    #[test]
    fn get_returns_none_when_uri_is_mismatch() -> () {
        let uri_map = UriMap::new();

        assert!(uri_map.get(&Uri::from_static("https://abc.api")).is_none());
    }
}