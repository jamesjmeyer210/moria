use regex::Regex;

pub struct UrlRegex {
    pub expr: Regex,
}

impl PartialEq for UrlRegex {
    fn eq(&self, other: &Self) -> bool {
        self.expr.as_str() == other.expr.as_str()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}