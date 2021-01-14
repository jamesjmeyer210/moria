use crate::Config;
use std::sync::{Arc, Mutex};
use crate::auth::url_map::UrlMap;
use actix_web::HttpRequest;
use crate::auth::traits::Authentication;

pub struct Authenticator {
    jwt_name: String,
    jwt_secret: String,
    endpoints: UrlMap,
}

impl Authenticator {
    pub fn new(jwt_name: String, jwt_secret: String, endpoints: UrlMap) -> Self {
        Authenticator {
            jwt_name,
            jwt_secret,
            endpoints,
        }
    }
}

impl Authentication<&HttpRequest> for Authenticator {
    fn authenticate(_: &HttpRequest) -> bool {
        unimplemented!()
    }
}