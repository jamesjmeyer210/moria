use regex::Regex;
use actix_web::http::Method;

struct Endpoint {
    url: Regex,
    groups: Vec<u8>,
    origin: u8,
}

struct UrlMap {
    groups: Vec<String>,
    origins: Vec<String>,
    endpoints: Vec<(Method, Vec<Endpoint>)>,
}