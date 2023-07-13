use crate::collection::Collection;

pub trait Stack<T>: Collection {
    fn push(&mut self, value: T);

    fn pop(&mut self) -> Option<T>;
}
