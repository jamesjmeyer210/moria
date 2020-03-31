#![macro_use]

use std::borrow::Borrow;

pub mod either;
pub mod jwt;

// A wrapper for the Vec type that only accepts unique elements
pub struct UniqueVec<T> {
    data: Vec<T>,
}

impl <T>UniqueVec<T> where T : PartialEq {

    pub fn new() -> Self {
        UniqueVec {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        UniqueVec {
            data: Vec::with_capacity(cap)
        }
    }

    pub fn push(&mut self, elem: T) {
        if !self.data.contains(&elem) {
            self.data.push(elem);
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.data
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_only_adds_unique_elements() {
        let names = vec![
            "Alice", "Bob", "Bob", "Charlie"
        ];

        let mut unique_names = UniqueVec::new();

        for name in names {
            unique_names.push(name);
        }

        let unique_names = unique_names.to_vec();

        assert_eq!(3, unique_names.len());
        assert_eq!("Alice".to_string(), unique_names.get(0).unwrap().to_string());
        assert_eq!("Bob".to_string(), unique_names.get(1).unwrap().to_string());
        assert_eq!("Charlie".to_string(), unique_names.get(2).unwrap().to_string());
    }

}