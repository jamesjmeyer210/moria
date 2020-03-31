use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum UrlType {
    Bool,
    Integer,
    Real,
    String,
}

impl UrlType {

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "{bool}" => Some(UrlType::Bool),
            "{integer}" => Some(UrlType::Integer),
            "{real}" => Some(UrlType::Real),
            "{string}" => Some(UrlType::String),
            _ => None
        }
    }

    pub fn get_regex_str(&self) -> &'static str {
        match self {
            UrlType::Bool => r"(true|false)",
            UrlType::Integer => r"[1-9][0-9]?",
            UrlType::Real => r"[1-9][0-9]?+\.+[0-9]",
            UrlType::String => r"/[\w\-\._~:/?#[\]@!\$&'\(\)\*\+,;=.]+$/",
        }
    }
}

#[cfg(test)]
mod test {

    use rand::prelude::*;
    use super::UrlType;

    #[test]
    fn from_str_returns_bool_when_given_bool() {
        let t = UrlType::from_str("{bool}").unwrap();
        assert_eq!(UrlType::Bool, t);
    }

    #[test]
    fn from_str_returns_integer_when_given_integer() {
        let t = UrlType::from_str("{integer}").unwrap();
        assert_eq!(UrlType::Integer, t);
    }

    #[test]
    fn from_str_returns_real_when_given_real() {
        let t = UrlType::from_str("{real}").unwrap();
        assert_eq!(UrlType::Real, t);
    }

    #[test]
    fn from_str_returns_string_when_given_string(){
        let t = UrlType::from_str("{string}").unwrap();
        assert_eq!(UrlType::String, t);
    }

    // #[test]
    // fn from_str_returns_string_when_given_lowercase_string() {
    //     let t = UrlType::from_str("string(1)").unwrap();
    //     assert_eq!(UrlType::String(1), t);
    // }
    //
    // #[test]
    // fn from_str_returns_string_when_given_camelcase_string(){
    //     let t = UrlType::from_str("String(1)").unwrap();
    //     assert_eq!(UrlType::String(1), t);
    // }
    //
    // #[test]
    // fn from_str_returns_string_with_size_when_given_string_of_n_size(){
    //     let r: usize = rand::thread_rng().gen_range(1, 9999);
    //
    //     let t = UrlType::from_str(&format!("String({})", r)).unwrap();
    //
    //     assert_eq!(UrlType::String(r), t);
    // }
    //
    // #[test]
    // fn from_str_returns_none_when_string_of_n_is_too_big(){
    //     let r: usize = rand::thread_rng().gen_range(10000, usize::max_value());
    //
    //     let t = UrlType::from_str(&format!("String({})", r));
    //
    //     assert_eq!(true, t.is_none());
    // }
    //
    // #[test]
    // fn get_regex_str_returns_string_of_any_usize(){
    //     let r: usize = rand::thread_rng().gen_range(1, 9999);
    //     let t = UrlType::from_str(&format!("String({})", r)).unwrap();
    //
    //     assert_eq!(r"/[\w\-\._~:/?#[\]@!\$&'\(\)\*\+,;=.]+$/", t.get_regex_str());
    // }
}