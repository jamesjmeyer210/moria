pub trait Authentication<T> {
    fn authenticate(_: T) -> bool;
}