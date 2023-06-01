use crate::AlgoError;
use std::fmt;
use std::fmt::Display;

// we can use high/depth of the tree, which we can return as a tuple for findRoot
// but I will stick with original approach as it is easier to use in future improvements
pub struct PathCompressionWeightedQuickUnion {
    objects: Vec<usize>,
    sizes: Vec<usize>,
}

impl PathCompressionWeightedQuickUnion {

    pub fn new(size: usize) -> PathCompressionWeightedQuickUnion {
        let mut objects: Vec<usize> = Vec::with_capacity(size.clone());
        let mut sizes: Vec<usize> = Vec::with_capacity(size.clone());
        for i in 0..size {
            objects.push(i);
            sizes.push(1);
        }
        return PathCompressionWeightedQuickUnion { objects, sizes };
    }

    fn find_root(&mut self, object: &usize) -> usize {
        let mut current = object.clone();
        while current.clone() != self.objects[current.clone()] {
            let parent = self.objects[current.clone()].clone();
            self.objects[current.clone()] = self.objects[parent.clone()].clone();
            current = parent;
        }
        return current;
    }

    pub fn add(&mut self, object: usize) -> Result<(), AlgoError> {
        if self.objects.get(object.clone()).is_some() {
            return Err(AlgoError::element_already_exist("object", &object));
        }

        self.objects.insert(object.clone(), object.clone());
        self.sizes.insert(object.clone(), 1);
        return Ok(());
    }

    pub fn union(&mut self, first: &usize, second: &usize) -> Result<(), AlgoError> {
        if self.objects.get(first.clone()).is_none() {
            return Err(AlgoError::missing_element("first object", first));
        }
        if self.objects.get(second.clone()).is_none() {
            return Err(AlgoError::missing_element("second object", second));
        }

        let first_root = self.find_root(first);
        let second_root = self.find_root(second);

        if self.sizes[first_root.clone()] < self.sizes[second_root.clone()] {
            self.objects[first_root.clone()] = second_root.clone();
            self.sizes[second_root.clone()] += self.sizes[first_root.clone()].clone()
        } else {
            self.objects[second_root.clone()] = first_root.clone();
            self.sizes[first_root.clone()] += self.sizes[second_root.clone()].clone()
        }

        return Ok(());
    }

    pub fn connected(&mut self, first: &usize, second: &usize) -> Result<bool, AlgoError> {
        if self.objects.get(first.clone()).is_none() {
            return Err(AlgoError::missing_element("first object", first));
        }
        if self.objects.get(second.clone()).is_none() {
            return Err(AlgoError::missing_element("second object", second));
        }

        let first_root = self.find_root(first);
        let second_root = self.find_root(second);
        return Ok(first_root == second_root);
    }
}

impl Display for PathCompressionWeightedQuickUnion {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for (index, value) in self.objects.iter().enumerate() {
            let res = write!(f, "{}:{}, ", index, value);
            if res.is_err() {
                return res;
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use easy_assert::bool_assertions::BooleanAssert;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::{actual_vec, expected_vec};
    use crate::dynamic_connectivity::path_compression_weighted_quick_union::PathCompressionWeightedQuickUnion;

    #[test]
    fn init_correctly() {
        let union_find = PathCompressionWeightedQuickUnion::new(10);

        ListAssert::assert_that(actual_vec(union_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
            .in_order();
    }

    #[test]
    fn adding_object_as_index_with_same_value() {
        let mut union_find = PathCompressionWeightedQuickUnion::new(5);
        // as index starting from 0, the next val would be 5

        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(union_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn adding_will_error_if_object_exist() {
        let mut union_find = PathCompressionWeightedQuickUnion::new(6);
        // as index starting from 0, the next val would be 6

        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_err()).is_true();

        ListAssert::assert_that(actual_vec(union_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_happy_path() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
            sizes: vec![1, 1, 1, 1, 1, 1],
        };

        let result = quick_union.union(&0, &3);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 0, 4, 5]))
            .in_order();
        ListAssert::assert_that(actual_vec(quick_union.sizes.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![2, 1, 1, 1, 1, 1]))
            .in_order();
    }

    #[test]
    fn union_long_happy_path() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            sizes: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        };
        let _ = quick_union.union(&0, &3);
        let _ = quick_union.union(&3, &4);
        let _ = quick_union.union(&3, &1);
        let _ = quick_union.union(&1, &2);
        let _ = quick_union.union(&5, &6);
        let _ = quick_union.union(&7, &8);
        let _ = quick_union.union(&6, &8);

        let result = quick_union.union(&1, &8);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 0, 0, 0, 0, 0, 5, 5, 5, 9]))
            .in_order();
        ListAssert::assert_that(actual_vec(quick_union.sizes.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![9, 1, 1, 1, 1, 4, 1, 2, 1, 1]))
            .in_order();
    }

    #[test]
    fn union_first_object_is_missing() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
            sizes: vec![1, 1, 1, 1, 1, 1],
        };

        let result = quick_union.union(&9, &3);

        BooleanAssert::assert_that(result.is_err()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_second_object_is_missing() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
            sizes: vec![1, 1, 1, 1, 1, 1],
        };

        let result = quick_union.union(&0, &9);

        BooleanAssert::assert_that(result.is_err()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_happy_several_existing_areas() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 2, 2, 5, 6, 6, 6, 8, 9],
            sizes: vec![1, 1, 3, 3, 3, 1, 3, 3, 3, 1, 1],
        };

        let result = quick_union.union(&2, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 2, 2, 5, 2, 6, 6, 8, 9]))
            .in_order();
    }

    #[test]
    fn union_happy_several_existing_areas_bigger_connected_to_small() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 2, 2, 3, 6, 6, 6, 8, 9],
            sizes: vec![1, 1, 4, 4, 4, 1, 3, 3, 3, 1, 1],
        };

        let result = quick_union.union(&2, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 2, 2, 3, 2, 6, 6, 8, 9]))
            .in_order();
    }

    #[test]
    fn connected_will_return_false_when_values_are_different() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4],
            sizes: vec![1, 1, 1, 1, 1],
        };

        let result = quick_union.connected(&0, &1);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_values_are_same() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 3, 3, 3, 4],
            sizes: vec![1, 3, 3, 3, 1],
        };

        let result = quick_union.connected(&1, &3);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_false_when_parents_are_different() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![2, 0, 2, 3, 4, 4, 5, 6],
            sizes: vec![3, 3, 3, 1, 4, 4, 4, 4],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_parents_are_same() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![2, 0, 4, 3, 4, 4, 5, 6],
            sizes: vec![7, 7, 7, 7, 7, 7, 7, 7],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_error_when_value_is_missing() {
        let mut quick_union = PathCompressionWeightedQuickUnion {
            objects: vec![0, 3, 3, 3, 4],
            sizes: vec![1, 3, 3, 3, 1],
        };

        let result = quick_union.connected(&1, &5);

        BooleanAssert::assert_that(result.is_err()).is_true();
    }
}
