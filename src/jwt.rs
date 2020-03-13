use jsonwebtoken::{decode, Validation, DecodingKey};
use jsonwebtoken::errors::ErrorKind;
use serde::{Serialize, Deserialize};
use actix_web::HttpRequest;

use crate::AuthObj;
use crate::Config;

#[derive(Serialize, Deserialize)]
struct JwtPayload {
    exp: i64,
    groups: Vec<String>,
}

#[derive(Debug)]
pub enum HeaderError {
    KeyNotFound,
    GroupNotFound,
    JwtError(ErrorKind),
}

// TODO: clean up this method so it doesn't have so many nested match blocks
pub fn validate_request(conf: &Config, req: &HttpRequest, auth_obj: &AuthObj) -> Result<(),HeaderError> {
    // If the AuthObject for the endpoint in question does not have any defined groups, no authentication
    // is required because none has been defined.
    if auth_obj.groups.is_empty(){
        return Ok(());
    }

    else if !req.headers().contains_key(&conf.jwt_key_name) {
        return Err(HeaderError::KeyNotFound);
    }
    // Because of the check above, this line should never fail
    let token = req.headers().get(&conf.jwt_key_name).unwrap();
    let token_data = decode::<JwtPayload>(
        &std::str::from_utf8(token.as_bytes()).unwrap(),
        &DecodingKey::from_secret(&conf.jwt_secret.as_bytes()),
        &Validation::default()
    );

    if token_data.is_err() {
        return Err(HeaderError::JwtError(token_data.err().unwrap().into_kind()));
    }
    // Extract the user from the token data
    let user = token_data.unwrap();
    // If there is one matching group between the user and the groups defined in the AuthObject, the
    // user belongs to a valid group to access this endpoint.
    for group in user.claims.groups.iter() {
        if auth_obj.groups.contains(group) {
            return Ok(());
        }
    }
    // When there are not intersections between the groups, the user may have a valid token, but they
    // have not been approved by the token issuer to access said endpoint
    return Err(HeaderError::GroupNotFound);
}

#[cfg(test)]
mod tests {
    // import the functions and struct from the higher level module
    use super::*;
    use actix_web::test;
    use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
    use chrono::prelude::*;

    fn default_config(jwt_key: &str, jwt_value: &str) -> Config {
        Config {
            jwt_key_name: jwt_key.to_string(),
            jwt_secret: jwt_value.to_string(),
            max_connection: 0,
            max_rate_of_connection: 0,
            timeout: 0,
            max_payload_size: 0,
        }
    }

    #[test]
    fn validate_request_returns_ok_when_auth_groups_is_empty(){
        // Create mock objects from the internal code base
        let conf = default_config("", "");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec![],
        };
        // Use actix web's test library to create mock requests
        let req = test::TestRequest::default().to_http_request();

        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn validate_request_returns_err_when_jwt_header_does_not_exist(){

        let conf = default_config("jwt-token", "");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            // Provide at lease one group so we can get past the first check
            groups: vec![
                "users".to_string()
            ],
        };

        let req = test::TestRequest::default().to_http_request();

        let result = validate_request(&conf, &req, &auth_obj);
        // In this case, we have to test the results of unwrap using string comparison because the
        // source library does not implement partial equal.
        assert_eq!("KeyNotFound", format!("{:?}", result.unwrap_err()))
    }

    #[test]
    fn validate_request_returns_err_when_jwt_value_is_invalid(){

        let conf = default_config("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec![
                "users".to_string()
            ],
        };

        let req = test::TestRequest::with_header("jwt-token", "wrong-secret")
            .to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!("JwtError(InvalidToken)", format!("{:?}", result.unwrap_err()))
    }

    #[test]
    fn validate_request_returns_ok_when_group_is_found(){

        let conf = default_config("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["users".to_string()],
        };

        let claims = JwtPayload {
            exp: Utc.timestamp(32503680000, 0).timestamp(),
            groups: vec!["users".to_string(), "admins".to_string()]
        };

        let token = encode::<JwtPayload>(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(conf.jwt_secret.as_bytes())
        ).unwrap();

        let req = test::TestRequest::with_header("jwt-token", token).to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn validate_request_returns_ok_when_group_is_found_in_raw_jwt(){

        let conf = default_config("jwt-token", "secret");

        let raw_jwt = format!("{}.{}.{}",
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9",
            "eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0",
            "8LGHRBirzKJPP4xhbyvIRLO-B7wMpUzJrOWgub4zASs"
        );

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec![
                "users".to_string()
            ],
        };

        let req = test::TestRequest::with_header("jwt-token", raw_jwt).to_http_request();

        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn validate_request_returns_err_when_group_is_not_found(){

        let conf = default_config("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["developers".to_string()],
        };

        let claims = JwtPayload {
            exp: Utc.timestamp(32503680000, 0).timestamp(),
            groups: vec!["users".to_string(), "admins".to_string()]
        };

        let token = encode::<JwtPayload>(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(conf.jwt_secret.as_bytes())
        ).unwrap();

        let req = test::TestRequest::with_header("jwt-token", token).to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!("GroupNotFound", format!("{:?}", result.unwrap_err()));
    }
}