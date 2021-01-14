use crate::Config;
use std::sync::{Arc, Mutex};
use crate::auth::url_map::UrlMap;
use actix_web::HttpRequest;
use crate::auth::traits::Authentication;
use actix_web::http::{HeaderMap, HeaderValue};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::model::{JwtPayload, AuthObj};
use actix_http::http::Uri;

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

    fn verify_header_value(&self, value: &HeaderValue) -> bool {
        match decode::<JwtPayload>(
            &std::str::from_utf8(value.as_bytes()).unwrap(),
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default())
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn verify_header_map(&self, map: &HeaderMap) -> bool {
        match map.get(&self.jwt_name) {
            Some(header) => self.verify_header_value(header),
            None => false,
        }
    }
}

impl Authentication<&HttpRequest> for Authenticator {
    fn authenticate(&self, req: &HttpRequest) -> bool {
        match self.verify_header_map(req.headers()) {
            true => true,
            _ => false,
        }
    }
}