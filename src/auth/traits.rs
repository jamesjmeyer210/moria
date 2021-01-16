pub trait Authentication<T> {
    fn authenticate(&self, _: T) -> bool;
}
