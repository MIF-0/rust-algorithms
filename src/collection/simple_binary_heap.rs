use std::fmt::Debug;

pub struct SimpleBinaryHeap<T>
where
    T: Ord + Debug,
{
    elements: Vec<T>,
}

impl<T> SimpleBinaryHeap<T>
where
    T: Ord + Debug,
{
    pub fn new() -> SimpleBinaryHeap<T> {
        SimpleBinaryHeap {
            elements: Vec::new(),
        }
    }
    pub fn add(&mut self, elem: T) {
        self.elements.push(elem);
        self.pop_up(self.elements.len() - 1);
    }

    pub fn delete_max(&mut self) -> T {
        let removed = self.elements.swap_remove(0);
        self.sink(0);

        removed
    }

    pub fn delete_min(&mut self) -> T {
        self.elements.remove(self.elements.len() - 1)
    }

    fn pop_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent_index = (index - 1) / 2;
        let current = &self.elements[index];
        let parent = &self.elements[parent_index];
        if current.gt(parent) {
            self.elements.swap(index, parent_index);
            self.pop_up(parent_index);
        }
    }

    fn sink(&mut self, index: usize) {
        if index >= self.elements.len() {
            return;
        }

        let left_child_index = SimpleBinaryHeap::<T>::left_child_index_for(index);
        let right_child_index = SimpleBinaryHeap::<T>::right_child_index_for(index);

        let left_child = self.elements.get(left_child_index);
        let right_child = self.elements.get(right_child_index);
        let current = &self.elements[index];
        match (left_child, right_child) {
            (Some(left_child), Some(right_child)) => {
                if left_child.gt(right_child) {
                    self.swap_if_child_gt(index, left_child_index);
                } else if right_child.gt(current) {
                    self.swap_if_child_gt(index, right_child_index);
                }
            }
            (Some(_), None) => {
                self.swap_if_child_gt(index, left_child_index);
            }
            (None, Some(_)) => {
                self.swap_if_child_gt(index, right_child_index);
            }
            (None, None) => {}
        }
    }

    fn swap_if_child_gt(&mut self, current_index: usize, child_index: usize) {
        let current = &self.elements[current_index];
        let child = &self.elements[child_index];
        if child.gt(current) {
            self.elements.swap(current_index, child_index);
            self.sink(child_index);
        }
    }

    fn left_child_index_for(index: usize) -> usize {
        (index * 2) + 1
    }

    fn right_child_index_for(index: usize) -> usize {
        (index * 2) + 2
    }
}
impl<T> Default for SimpleBinaryHeap<T>
where
    T: Ord + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::collection::simple_binary_heap::SimpleBinaryHeap;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::string_assertions::StringAssert;
    use easy_assert::{actual, actual_vec, expected, expected_vec};

    #[test]
    fn basics_add() {
        let array = vec!["T", "P", "R", "N", "H", "O", "A", "E", "I", "G", "S"];

        let mut binary_heap: SimpleBinaryHeap<&str> = SimpleBinaryHeap::new();
        for value in array {
            binary_heap.add(value);
        }
        ListAssert::assert_that(actual_vec(binary_heap.elements))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![
                "T", "S", "R", "N", "P", "O", "A", "E", "I", "G", "H",
            ]))
            .in_order();
    }

    #[test]
    fn basics_delete_min() {
        let array = vec!["T", "P", "R", "N", "H", "O", "A", "E", "I", "G", "S"];

        let mut binary_heap: SimpleBinaryHeap<&str> = SimpleBinaryHeap::new();
        for value in array {
            binary_heap.add(value);
        }

        let removed = binary_heap.delete_min();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("H".to_string()));

        ListAssert::assert_that(actual_vec(binary_heap.elements))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![
                "T", "S", "R", "N", "P", "O", "A", "E", "I", "G",
            ]))
            .in_order();
    }

    #[test]
    fn basics_delete_max() {
        let array = vec!["T", "P", "R", "N", "H", "O", "A", "E", "I", "G", "S"];

        let mut binary_heap: SimpleBinaryHeap<&str> = SimpleBinaryHeap::new();
        for value in array {
            binary_heap.add(value);
        }

        let removed = binary_heap.delete_max();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("T".to_string()));

        ListAssert::assert_that(actual_vec(binary_heap.elements))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![
                "S", "P", "R", "N", "H", "O", "A", "E", "I", "G",
            ]))
            .in_order();
    }

    #[test]
    fn basics_delete_max_n() {
        let array = vec!["T", "P", "R", "N", "H", "O", "A", "E", "I", "G", "S"];

        let mut binary_heap: SimpleBinaryHeap<&str> = SimpleBinaryHeap::new();
        for value in array {
            binary_heap.add(value);
        }

        let removed = binary_heap.delete_max();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("T".to_string()));
        let removed = binary_heap.delete_max();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("S".to_string()));

        ListAssert::assert_that(actual_vec(binary_heap.elements))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![
                "R", "P", "O", "N", "H", "G", "A", "E", "I",
            ]))
            .in_order();
    }

    #[test]
    fn basics_delete_max_n_add() {
        let array = vec!["T", "P", "R", "N", "H", "O", "A", "E", "I", "G", "S"];

        let mut binary_heap: SimpleBinaryHeap<&str> = SimpleBinaryHeap::new();
        for value in array {
            binary_heap.add(value);
        }

        let removed = binary_heap.delete_max();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("T".to_string()));
        let removed = binary_heap.delete_max();
        StringAssert::assert_that(actual(removed.to_string()))
            .is_equal()
            .to(expected("S".to_string()));

        binary_heap.add("S");
        ListAssert::assert_that(actual_vec(binary_heap.elements))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![
                "S", "R", "O", "N", "P", "G", "A", "E", "I","H"
            ]))
            .in_order();

    }
}
