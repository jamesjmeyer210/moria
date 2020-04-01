#![macro_use]

use std::borrow::Borrow;
use regex::Regex;

pub mod either;
pub mod jwt;

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

mod unique_vec;
pub type UniqueVec<T> = unique_vec::UniqueVec<T>;