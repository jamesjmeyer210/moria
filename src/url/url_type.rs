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

    // pub fn extract_string(s: &str) -> Option<UrlType> {
    //     let reg_string: Regex = Regex::new(r"(S|s)+tring+\(+[1-9][0-9]{0,3}?\)").unwrap();
    //     match reg_string.captures(s) {
    //         Some(capture) => {
    //             // Extract the selected text from captured value of the regex expression
    //             let text: &str = capture.get(0).unwrap().as_str();
    //             let reg_usize: Regex = Regex::new(r"[1-9][0-9]{0,3}").unwrap();
    //
    //             // The bellow unwrap errors are unreachable because we cannot enter this
    //             // code block without already having a valid selection. The outer regex
    //             // rules are more strict than the internal rules.
    //             let str_usize = reg_usize.captures(text).unwrap().get(0).unwrap().as_str();
    //
    //             // A ParseIntError is unreachable in the blow line because we are extracting
    //             // this usize after we've captured it in a regex expression.
    //             let size = usize::from_str(str_usize).unwrap();
    //
    //             Some(UrlType::String(size))
    //         },
    //         _ => None
    //     }
    // }
}

#[cfg(test)]
mod test {

    use rand::prelude::*;
    use super::UrlType;

    #[test]
    fn from_str_returns_bool_when_given_bool() {
        let t = UrlType::from_str("bool").unwrap();
        assert_eq!(UrlType::Bool, t);
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