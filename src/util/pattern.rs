use std::str::FromStr;
use regex::{Regex, Error};

pub struct Pattern<T> {
    expr: T,
}

impl FromStr for Pattern<Regex> {
    type Err = Error;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq_str_returns_true_if_regex_captures() {
        // let r = UrlRegex::from_str(r"[a-z]+").unwrap();
        // assert_eq!(true, r.eq_str("alice"))
        let r: Pattern<Regex> = Pattern::from_str(r"[a-z]+").unwrap();
        assert_eq!(true, r.matches_str("alice"))
    }

    #[test]
    fn eq_str_returns_true_if_regex_captures_complex(){
        let r: Pattern<Regex> = Pattern::from_str(r"[a-zA-Z0-9]+").unwrap();
        assert_eq!(true, r.matches_str("Al1c3"));
    }

    #[test]
    fn eq_str_returns_false_if_regex_captures_not_exact(){
        let r: Pattern<Regex> = Pattern::from_str(r"[a-zA-Z0-9]+").unwrap();
        assert_eq!(false, r.matches_str("Al1c3_B0b"));
    }
}