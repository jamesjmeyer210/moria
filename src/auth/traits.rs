pub trait Authorization<T> {
    fn authorize(&self, _: T) -> bool;
}
