use regex::Regex;

enum ConversionError {
    InvalidPattern,
    UnknownType,
}

const PATH_DEFINITION: &'static str = r"[[/]([a-zA-Z0-9]+|\{+(bool|integer|real|string)+\})]+";

fn is_valid_pattern(url: &str) -> bool {
    let reg: Regex = Regex::new(PATH_DEFINITION).unwrap();
    match reg.captures(url) {
        Some(pattern) => {
            let r = pattern.get(0).unwrap().as_str();
            println!("DEBUG: {}", r);
            url == r
        },
        None => false
    }

}

// fn convert(url: &String) -> Result<Regex,ConversionError> {
//
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pattern_with_no_params_is_valid() {
        let path = "/api";
        assert_eq!(true, is_valid_pattern(path));
    }

    #[test]
    fn pattern_with_depth_is_valid() {
        let path = "/api/users";
        assert_eq!(true, is_valid_pattern(path));
    }

    #[test]
    fn pattern_with_bool_is_valid() {
        let path = "/api/exists/{bool}";
        assert_eq!(true, is_valid_pattern(path));
    }

    #[test]
    fn pattern_with_integer_is_valid() {
        let path = "/api/user/{integer}";
        assert_eq!(true, is_valid_pattern(path));
    }
}