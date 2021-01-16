use crate::auth::traits::Authentication;
use crate::auth::url_map::UriMap;
use crate::model::{AuthObj, JwtPayload};
use crate::Config;
use actix_http::http::Uri;
use actix_web::http::{HeaderMap, HeaderValue};
use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use std::sync::{Arc, Mutex};

pub struct Authenticator {
    jwt_name: String,
    jwt_secret: String,
    endpoints: UriMap,
}

impl Authenticator {
    pub fn new(jwt_name: String, jwt_secret: String, endpoints: UriMap) -> Self {
        Authenticator {
            jwt_name,
            jwt_secret,
            endpoints,
        }
    }

    fn verify_header_value(&self, value: &HeaderValue) -> Option<TokenData<JwtPayload>> {
        match decode::<JwtPayload>(
            &std::str::from_utf8(value.as_bytes()).unwrap(),
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(header) => Some(header),
            _ => None,
        }
    }

    fn verify_header_map(&self, header_map: &HeaderMap) -> Option<Vec<String>> {
        header_map
            .get(&self.jwt_name)
            .and_then(|header_value| self.verify_header_value(header_value))
            .and_then(|jwt| Some(jwt.claims.groups))
    }
}

impl Authentication<&HttpRequest> for Authenticator {
    fn authenticate(&self, req: &HttpRequest) -> bool {
        // First, try to get our groups based off their uri
        let groups = self.endpoints.get(req.uri());
        if groups.is_none() {
            return false;
        }
        // Second, try to get their claimed groups, by verifying out signature key after theirs
        let claimed_groups = self.verify_header_map(req.headers());
        if claimed_groups.is_none() {
            return false;
        }
        // Third, check to see if some of our groups match theirs
        let groups = groups.unwrap();
        for cg in claimed_groups.unwrap().iter() {
            if groups.contains(cg) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;

    #[test]
    fn verify_header_returns_some_when_value_is_valid() -> () {
        let auth = Authenticator::new("".to_string(), "secret".to_string(), UriMap::new());

        let token = HeaderValue::from_static(
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
        .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
        .8LGHRBirzKJPP4xhbyvIRLO-B7wMpUzJrOWgub4zASs",
        );

        assert!(auth.verify_header_value(&token).is_some());
    }

    #[test]
    fn verify_header_value_returns_none_when_signature_does_not_match() -> () {
        let auth = Authenticator::new("".to_string(), "secret".to_string(), UriMap::new());

        let token = HeaderValue::from_static(
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
        .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
        .XO1xiMZljpvAOsXPGEKSJmyfgcUum7nOmUmw63kzyio",
        );

        assert!(auth.verify_header_value(&token).is_none());
    }

    #[test]
    fn verify_header_map_returns_none_when_key_does_not_exist() -> () {
        let req = test::TestRequest::with_header("blue", "missing").to_http_request();

        let auth = Authenticator::new("green".to_string(), "secret".to_string(), UriMap::new());

        let header = auth.verify_header_map(req.headers());
        assert!(header.is_none());
    }

    #[test]
    fn verify_header_map_returns_none_jwt_formatting_is_illegal() -> () {
        let req =
            test::TestRequest::with_header("green", "illegal/\"header@value").to_http_request();

        let auth = Authenticator::new("green".to_string(), "secret".to_string(), UriMap::new());

        let header = auth.verify_header_map(req.headers());
        assert!(header.is_none());
    }

    #[test]
    fn verify_header_map_returns_none_when_signature_is_invalid() -> () {
        let req = test::TestRequest::with_header(
            "green",
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
            .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
            .XO1xiMZljpvAOsXPGEKSJmyfgcUum7nOmUmw63kzyio",
        )
        .to_http_request();

        let auth = Authenticator::new("green".to_string(), "secret".to_string(), UriMap::new());

        let header = auth.verify_header_map(req.headers());
        assert!(header.is_none());
    }

    #[test]
    fn verify_header_map_returns_groups_when_signature_is_valid() -> () {
        let req = test::TestRequest::with_header(
            "green",
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
            .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
            .8LGHRBirzKJPP4xhbyvIRLO-B7wMpUzJrOWgub4zASs",
        )
        .to_http_request();

        let auth = Authenticator::new("green".to_string(), "secret".to_string(), UriMap::new());

        let header = auth.verify_header_map(req.headers());
        assert!(header.is_some());
    }
}
