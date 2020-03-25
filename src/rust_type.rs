use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum RustType {
    BOOL,
    I8,
    U8,
    CHAR,
    I16,
    U16,
    I32,
    U32,
    F32,
    I64,
    U64,
    F64,
    String(usize),
}

impl RustType {

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "bool" => Some(RustType::BOOL),
            "i8"|"sbyte" => Some(RustType::I8),
            "u8"|"byte" => Some(RustType::U8),
            "char" => Some(RustType::CHAR),
            "i16"|"short" => Some(RustType::I16),
            "u16"|"ushort" => Some(RustType::U16),
            "i32"|"int" => Some(RustType::I32),
            "u32"|"uint" => Some(RustType::U32),
            "f32"|"double" => Some(RustType::F32),
            "i64"|"long" => Some(RustType::I64),
            "u64"|"ulong" => Some(RustType::U64),
            "f64"|"decimal" => Some(RustType::F64),
            _ => RustType::extract_string(s)
        }
    }

    fn extract_string(s: &str) -> Option<RustType> {
        let reg_string: Regex = Regex::new(r"(S|s)+tring+\(+[1-9][0-9]{0,3}?\)").unwrap();
        match reg_string.captures(s) {
            Some(capture) => {
                // Extract the selected text from captured value of the regex expression
                let text: &str = capture.get(0).unwrap().as_str();
                let reg_usize: Regex = Regex::new(r"[1-9][0-9]{0,3}").unwrap();

                // The bellow unwrap errors are unreachable because we cannot enter this
                // code block without already having a valid selection. The outer regex
                // rules are more strict than the internal rules.
                let str_usize = reg_usize.captures(text).unwrap().get(0).unwrap().as_str();

                // A ParseIntError is unreachable in the blow line because we are extracting
                // this usize after we've captured it in a regex expression.
                let size = usize::from_str(str_usize).unwrap();

                Some(RustType::String(size))
            },
            _ => None
        }
    }
}

#[cfg(test)]
mod test {

    use rand::prelude::*;
    use super::RustType;

    #[test]
    fn from_str_returns_bool_when_given_bool() {
        let t = RustType::from_str("bool").unwrap();
        assert_eq!(RustType::BOOL, t);
    }

    #[test]
    fn from_str_returns_string_when_given_lowercase_string() {
        let t = RustType::from_str("string(1)").unwrap();
        assert_eq!(RustType::String(1), t);
    }

    #[test]
    fn from_str_returns_string_when_given_camelcase_string(){
        let t = RustType::from_str("String(1)").unwrap();
        assert_eq!(RustType::String(1), t);
    }

    #[test]
    fn from_str_returns_string_with_size_when_given_string_of_n_size(){
        let r: usize = rand::thread_rng().gen_range(1, 9999);

        let t = RustType::from_str(&format!("String({})", r)).unwrap();

        assert_eq!(RustType::String(r), t);
    }

    #[test]
    fn from_str_returns_none_when_string_of_n_is_too_big(){
        let r: usize = rand::thread_rng().gen_range(10000, usize::max_value());

        let t = RustType::from_str(&format!("String({})", r));

        assert_eq!(true, t.is_none());
    }
}