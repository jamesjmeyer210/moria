use regex::Regex;

// TODO:
// This struct probably belong with the other url types and should possibly to private within the
// url package - maybe it should be used underneath the url_type structure.
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