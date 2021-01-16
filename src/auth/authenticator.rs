use crate::Config;
use std::sync::{Arc, Mutex};
use crate::auth::url_map::UriMap;
use actix_web::HttpRequest;
use crate::auth::traits::Authentication;
use actix_web::http::{HeaderMap, HeaderValue};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use crate::model::{JwtPayload, AuthObj};
use actix_http::http::Uri;

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

    fn verify_header_value(&self, value: &HeaderValue) -> Result<TokenData<JwtPayload>,()> {
        match decode::<JwtPayload>(
            &std::str::from_utf8(value.as_bytes()).unwrap(),
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default())
        {
            Ok(header) => Ok(header),
            Err(_) => (),
        }
    }

    fn verify_header_map(&self, map: &HeaderMap) -> Option<JwtPayload> {
        // match map.get(&self.jwt_name) {
        //     Some(header) => self.verify_header_value(header),
        //     None => false,
        // }

        // let y = map.get(&self.jwt_name).iter().map(|x|{
        //     decode::<JwtPayload>(
        //         &std::str::from_utf8(x.as_bytes()).unwrap(),
        //         &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
        //         &Validation::default())
        // }).collect();

        let a = map.get(&self.jwt_name);
        let b = a.and_then(|value|{
                match decode::<JwtPayload>(
                    &std::str::from_utf8(x.as_bytes()).unwrap(),
                    &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
                    &Validation::default()) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            });
        let c = b.and_then(|jwt|{
           Some(jwt.claims.groups)
        });


        None
    }
}

impl Authentication<&HttpRequest> for Authenticator {
    fn authenticate(&self, req: &HttpRequest) -> bool {
        // match self.verify_header_map(req.headers()) {
        //     true => true,
        //     _ => false,
        // }
        let d = self.endpoints.get(req.uri()).and_then(|groups|{
            let a = map.get(&self.jwt_name)
                .and_then(|value|{
                match decode::<JwtPayload>(
                    &std::str::from_utf8(x.as_bytes()).unwrap(),
                    &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
                    &Validation::default()) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            }).and_then(|jwt|{
                Some(jwt.claims.groups)
            });

            Some(())
        });

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_header_value_returns_true_when_value_is_valid() -> () {
        let auth = Authenticator::new("".to_string()
                                      , "secret".to_string()
                                      , UriMap::new());

        let token = HeaderValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
        .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
        .8LGHRBirzKJPP4xhbyvIRLO-B7wMpUzJrOWgub4zASs");

        assert!(auth.verify_header_value(&token));
    }

    #[test]
    fn verify_header_value_returns_false_when_signature_does_not_match() -> () {
        let auth = Authenticator::new("".to_string()
                                      , "secret".to_string()
                                      , UriMap::new());

        let token = HeaderValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
        .eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0\
        .XO1xiMZljpvAOsXPGEKSJmyfgcUum7nOmUmw63kzyio");

        assert!(!auth.verify_header_value(&token));
    }

    fn verify_header_map
}