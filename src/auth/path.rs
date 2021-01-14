use std::rc::Rc;
use actix_http::http::Uri;
use crate::auth::traits::{Authentication};

pub struct Path {
    uri: Rc<Uri>,
    groups: Vec<String>,
}