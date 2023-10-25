use crate::collection::simple_binary_heap::SimpleBinaryHeap;
use std::fmt::Debug;

pub fn sort<T>(array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    SimpleBinaryHeap::from(array).sort()
}
