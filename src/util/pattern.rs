use std::str::FromStr;
use regex::Regex;

pub struct Pattern<T> {
    expr: T,
}

impl FromStr for Pattern<Regex> {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::from_str(s);
        if r.is_err() {
            Err(r.unwrap_err())
        }
        else {
            Ok(Pattern{ expr: r.unwrap() })
        }
    }
}

impl FromStr for Pattern<String> {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern{ expr: s.to_string() })
    }
}

trait MatchStr {
    fn matches_str(&self, s: &str) -> bool;
}

impl MatchStr for Pattern<Regex> {
    fn matches_str(&self, s: &str) -> bool {
        match self.expr.captures(s) {
            Some(c) => {
                if c.len() == 0 {
                    false
                }
                else {
                    s == c.get(0).unwrap().as_str()
                }
            },
            None => false
        }
    }
}

impl MatchStr for Pattern<&str> {
    fn matches_str(&self, s: &str) -> bool {
        self.expr == s
    }
}

impl MatchStr for Pattern<String> {
    fn matches_str(&self, s: &str) -> bool {
        self.expr.as_str() == s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_str_returns_true_if_regex_captures() {
        let r: Pattern<Regex> = Pattern::from_str(r"[a-z]+").unwrap();
        assert_eq!(true, r.matches_str("alice"))
    }

    #[test]
    fn matches_str_returns_true_if_regex_captures_complex(){
        let r: Pattern<Regex> = Pattern::from_str(r"[a-zA-Z0-9]+").unwrap();
        assert_eq!(true, r.matches_str("Al1c3"));
    }

    #[test]
    fn matches_str_returns_false_if_regex_captures_not_exact(){
        let r: Pattern<Regex> = Pattern::from_str(r"[a-zA-Z0-9]+").unwrap();
        assert_eq!(false, r.matches_str("Al1c3_B0b"));
    }

    #[test]
    fn matches_str_returns_true_if_equal(){
        let p: Pattern<String> = Pattern::from_str("Alice").unwrap();
        assert_eq!(true, p.matches_str("Alice"));
    }

    #[test]
    fn matches_str_returns_false_if_not_equal(){
        let p: Pattern<String> = Pattern::from_str("Bob").unwrap();
        assert_eq!(false, p.matches_str("Alice"));
    }
}