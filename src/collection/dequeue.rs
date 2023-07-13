use crate::collection::Collection;

pub trait Dequeue<T>: Collection {
    fn push_front(&mut self, value: T);

    fn pop_front(&mut self) -> Option<T>;

    fn push_back(&mut self, value: T);

    fn pop_back(&mut self) -> Option<T>;
}
