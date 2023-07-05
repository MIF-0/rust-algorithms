pub mod different_linked_list;
pub mod first_attempt_linked_list;
pub mod stack;

pub trait Collection {
    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}
