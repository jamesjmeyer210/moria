use regex::{Regex, Error};
use std::str::FromStr;

// TODO:
// This struct probably belong with the other url types and should possibly to private within the
// url package - maybe it should be used underneath the url_type structure.
pub struct UrlRegex {
    pub expr: Regex,
}

trait EqStr {
    fn eq_str(&self, other: &str) -> bool;
    fn ne_str(&self, other: &str) -> bool;
}

impl FromStr for UrlRegex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::from_str(s);
        if r.is_err() {
            Err(r.unwrap_err())
        }
        else {
            Ok(UrlRegex{ expr: r.unwrap() })
        }
    }
}

impl EqStr for UrlRegex {
    fn eq_str(&self, other: &str) -> bool {
        match self.expr.captures(other) {
            Some(capture) => {
                if capture.len() == 0 {
                    println!("DEBUG: capture len is 0");
                    false
                }
                else {
                    other == capture.get(0).unwrap().as_str()
                }
            },
            None => {
                println!("DEBUG: no captures");
                false
            }
        }
    }

    fn ne_str(&self, other: &str) -> bool {
        !self.eq_str(other)
    }
}

impl PartialEq for UrlRegex {
    fn eq(&self, other: &Self) -> bool {
        self.expr.as_str() == other.expr.as_str()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}