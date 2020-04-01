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

    pub fn push(&mut self, elem: T) -> usize {
        if !self.data.contains(&elem) {
            self.data.push(elem);
            self.data.len() - 1
        }
        else {
            self.index_of(&elem).unwrap()
        }
    }

    fn index_of(&self, target: &T) -> Option<usize> {
        let mut i: usize = 0;
        for elem in self.data.iter() {
            if elem == target {
                return Some(i);
            }
            i+=1;
        }
        None
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

    #[test]
    fn push_returns_index_of_zero_on_first_push() {
        let mut unique = UniqueVec::new();
        let index = unique.push("first");
        assert_eq!(0, index);
    }

    #[test]
    fn index_of_returns_unique_index_when_element_exists() {
        let names = vec![
            "Alice", "Bob", "Bob", "Charlie"
        ];

        let mut unique_names = UniqueVec::new();

        for name in names {
            unique_names.push(name);
        }

        assert_eq!(0, unique_names.index_of(&"Alice").unwrap());
        assert_eq!(1, unique_names.index_of(&"Bob").unwrap());
        assert_eq!(2, unique_names.index_of(&"Charlie").unwrap());
    }
}