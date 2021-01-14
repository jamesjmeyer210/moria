use crate::model::{AuthObj, ConfigError, Domain, JwtPayload};
use actix_web::client::Client;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashMap;
use std::fs;
use std::str;

#[macro_use]
mod util;
mod model;
mod auth;
mod proxy;

pub type Config = model::Config;

async fn send(
    client: &Client,
    url: &str,
    req: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    // Build the client request for the proxy
    let mut forwarded_req = client.request_from(url, req.head()).no_decompress();
    // Copy the header values from the incoming request to
    // the forwarded request.
    for (header_name, header_value) in req.headers().iter() {
        forwarded_req = forwarded_req.set_header(header_name.clone(), header_value.clone());
    }
    // finally, send the request and return any errors if we get them
    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    // Build the response status of the proxy
    let mut client_resp = HttpResponse::build(res.status());
    // Add the response's headers
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }
    // Return our constructed response
    Ok(client_resp.body(res.body().await?))
}

pub async fn forward(
    config: web::Data<Config>,
    endpoints: web::Data<HashMap<String, AuthObj>>,
    client: web::Data<Client>,
    req: HttpRequest,
    body: web::Bytes,
) -> impl Responder {
    let lookup = format!("{} {}", req.method(), req.path());

    match endpoints.get(&lookup) {
        Some(endpoint) => match validate_request(&config, &req, &endpoint) {
            Ok(()) => {
                let url = format!("{}{}", endpoint.origin, req.path());

                send(&client, &url, req, body)
                    .await
                    .unwrap_or_else(|error| {
                        println!("{}", error);
                        HttpResponse::InternalServerError().finish()
                    })
            }
            Err(error) => {
                println!("{} {:?}", lookup, error);
                HttpResponse::Unauthorized().finish()
            }
        },
        None => HttpResponse::NotFound().body(lookup),
    }
}

pub fn load_endpoints(path: &str) -> HashMap<String, AuthObj> {
    let domains: Vec<Domain> = load_json_file!(path);

    let mut map: HashMap<String, AuthObj> = HashMap::new();

    for domain in domains {
        for endpoint in domain.endpoints {
            let key = format!("{} {}", endpoint.method, &endpoint.path);
            map.insert(key, AuthObj::new(domain.origin.clone(), endpoint.groups));
        }
    }

    return map;
}

#[derive(Debug)]
pub enum HeaderError {
    KeyNotFound,
    GroupNotFound,
    JwtError(ErrorKind),
}

// TODO: clean up this method so it doesn't have so many nested match blocks
pub fn validate_request(
    conf: &Config,
    req: &HttpRequest,
    auth_obj: &AuthObj,
) -> Result<(), HeaderError> {
    // If the AuthObject for the endpoint in question does not have any defined groups, no authentication
    // is required because none has been defined.
    if auth_obj.groups.is_empty() {
        return Ok(());
    } else if !req.headers().contains_key(&conf.jwt_key_name) {
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
mod test {

    use super::*;
    use actix_web::test;
    use chrono::prelude::*;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    macro_rules! before_config {
        ($var:ident, $val:expr, $path:expr) => {{
            let mut invalid_config = Config::default();
            invalid_config.$var = $val;

            let json = serde_json::to_string(&invalid_config).unwrap();
            fs::write($path, json).unwrap();
        }};
    }

    macro_rules! after_config {
        ($path:expr) => {{
            fs::remove_file($path).unwrap();
        }};
    }

    #[test]
    #[should_panic]
    fn from_file_fails_on_invalid_path() {
        let _config = Config::from_file("/dev/null/config.json");
    }

    #[test]
    #[should_panic]
    fn from_file_fails_on_invalid_json() {
        let _config = Config::from_file("test/test.json");
    }

    #[test]
    fn from_file_returns_invalid_port_when_port_is_zero() {
        before_config!(port, 0, "/tmp/invalid_port.json");
        let error = Config::from_file("/tmp/invalid_port.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPort, error);

        after_config!("/tmp/invalid_port.json")
    }

    #[test]
    fn from_file_returns_invalid_private_ssl_key_when_file_not_found() {
        before_config!(
            ssl_public_key,
            "test/ssl/key.pem".to_string(),
            "/tmp/invalid_private_ssl_key.json"
        );
        let error = Config::from_file("/tmp/invalid_private_ssl_key.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPublicKey, error);

        after_config!("/tmp/invalid_private_ssl_key.json");
    }

    #[test]
    fn from_file_returns_invalid_public_ssl_key_when_file_not_found() {
        before_config!(
            ssl_private_key,
            "test/ssl/cert.pem".to_string(),
            "/tmp/invalid_public_ssl_key.json"
        );
        let error = Config::from_file("/tmp/invalid_public_ssl_key.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPrivateKey, error);

        after_config!("/tmp/invalid_public_ssl_key.json");
    }

    #[test]
    fn from_file_returns_invalid_max_connection_when_value_is_zero() {
        before_config!(max_connection, 0, "/tmp/invalid_max_connection.json");
        let error = Config::from_file("/tmp/invalid_max_connection.json").unwrap_err();

        assert_eq!(ConfigError::InvalidMaxConnection, error);

        after_config!("/tmp/invalid_max_connection.json");
    }

    #[test]
    fn from_file_returns_invalid_max_rate_of_connection_when_value_is_zero() {
        before_config!(
            max_rate_of_connection,
            0,
            "/tmp/invalid_max_rate_of_connection.json"
        );
        let error = Config::from_file("/tmp/invalid_max_rate_of_connection.json").unwrap_err();

        assert_eq!(ConfigError::InvalidMaxRateOfConnection, error);

        after_config!("/tmp/invalid_max_rate_of_connection.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_path() {
        let _config = Config::from_file("/dev/null/endpoints.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_json() {
        let _config = Config::from_file("test/test.json");
    }

    #[test]
    fn validate_request_returns_ok_when_auth_groups_is_empty() {
        // Create mock objects from the internal code base
        let conf = Config::from_jwt_key_value("", "");

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
    fn validate_request_returns_err_when_jwt_header_does_not_exist() {
        let conf = Config::from_jwt_key_value("jwt-token", "");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            // Provide at lease one group so we can get past the first check
            groups: vec!["users".to_string()],
        };

        let req = test::TestRequest::default().to_http_request();

        let result = validate_request(&conf, &req, &auth_obj);
        // In this case, we have to test the results of unwrap using string comparison because the
        // source library does not implement partial equal.
        assert_eq!("KeyNotFound", format!("{:?}", result.unwrap_err()))
    }

    #[test]
    fn validate_request_returns_err_when_jwt_value_is_invalid() {
        let conf = Config::from_jwt_key_value("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["users".to_string()],
        };

        let req = test::TestRequest::with_header("jwt-token", "wrong-secret").to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!(
            "JwtError(InvalidToken)",
            format!("{:?}", result.unwrap_err())
        )
    }

    #[test]
    fn validate_request_returns_ok_when_group_is_found() {
        let conf = Config::from_jwt_key_value("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["users".to_string()],
        };

        let claims = JwtPayload {
            exp: Utc.timestamp(32503680000, 0).timestamp(),
            groups: vec!["users".to_string(), "admins".to_string()],
        };

        let token = encode::<JwtPayload>(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(conf.jwt_secret.as_bytes()),
        )
        .unwrap();

        let req = test::TestRequest::with_header("jwt-token", token).to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn validate_request_returns_ok_when_group_is_found_in_raw_jwt() {
        let conf = Config::from_jwt_key_value("jwt-token", "secret");

        let raw_jwt = format!(
            "{}.{}.{}",
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9",
            "eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0",
            "8LGHRBirzKJPP4xhbyvIRLO-B7wMpUzJrOWgub4zASs"
        );

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["users".to_string()],
        };

        let req = test::TestRequest::with_header("jwt-token", raw_jwt).to_http_request();

        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn validate_request_returns_err_when_group_is_not_found() {
        let conf = Config::from_jwt_key_value("jwt-token", "secret");

        let auth_obj = AuthObj {
            origin: "".to_string(),
            groups: vec!["developers".to_string()],
        };

        let claims = JwtPayload {
            exp: Utc.timestamp(32503680000, 0).timestamp(),
            groups: vec!["users".to_string(), "admins".to_string()],
        };

        let token = encode::<JwtPayload>(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(conf.jwt_secret.as_bytes()),
        )
        .unwrap();

        let req = test::TestRequest::with_header("jwt-token", token).to_http_request();
        let result = validate_request(&conf, &req, &auth_obj);

        assert_eq!("GroupNotFound", format!("{:?}", result.unwrap_err()));
    }
}
