use crate::dynamic_connectivity::union_find::UnionFind;
use crate::AlgoError;
use std::fmt;
use std::fmt::Display;

// we can use high/depth of the tree, which we can return as a tuple for findRoot
// but I will stick with original approach as it is easier to use in future improvements
pub struct WeightedQuickUnion {
    objects: Vec<usize>,
    sizes: Vec<usize>,
}

impl WeightedQuickUnion {
    pub fn new(size: usize) -> Self {
        let mut objects: Vec<usize> = Vec::with_capacity(size);
        let mut sizes: Vec<usize> = Vec::with_capacity(size);
        for i in 0..size {
            objects.push(i);
            sizes.push(1);
        }

        WeightedQuickUnion { objects, sizes }
    }

    fn find_root(&self, object: &usize) -> Option<usize> {
        let possible_parent = self.objects.get(*object);

        match possible_parent {
            None => None,
            Some(parent) => {
                if object == parent {
                    Some(*parent)
                } else {
                    self.find_root(parent)
                }
            }
        }
    }
}

impl UnionFind for WeightedQuickUnion {
    fn add(&mut self, object: usize) -> Result<(), AlgoError> {
        if self.objects.get(object).is_some() {
            return Err(AlgoError::element_already_exist("object", &object));
        }

        self.objects.insert(object, object);
        self.sizes.insert(object, 1);

        Ok(())
    }

    fn union(&mut self, first: &usize, second: &usize) -> Result<(), AlgoError> {
        let first_root = self.find_root(first);
        let second_root = self.find_root(second);

        match (first_root, second_root) {
            (Some(f), Some(s)) => {
                if self.sizes[f] < self.sizes[s] {
                    self.objects[f] = s;
                    self.sizes[s] += self.sizes[f]
                } else {
                    self.objects[s] = f;
                    self.sizes[f] += self.sizes[s]
                }

                Ok(())
            }
            (None, None) => Err(AlgoError::missing_elements(
                "first object",
                first,
                "second object",
                second,
            )),
            (None, _) => Err(AlgoError::missing_element("first object", first)),
            (_, None) => Err(AlgoError::missing_element("second object", second)),
        }
    }

    fn connected(&self, first: &usize, second: &usize) -> Result<bool, AlgoError> {
        let first_root = self.find_root(first);
        let second_root = self.find_root(second);

        match (first_root, second_root) {
            (Some(f), Some(s)) => Ok(f == s),
            (None, None) => Err(AlgoError::missing_elements(
                "first object",
                first,
                "second object",
                second,
            )),
            (None, _) => Err(AlgoError::missing_element("first object", first)),
            (_, None) => Err(AlgoError::missing_element("second object", second)),
        }
    }
}

impl Display for WeightedQuickUnion {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for (index, value) in self.objects.iter().enumerate() {
            let res = write!(f, "{}:{}, ", index, value);
            res?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dynamic_connectivity::test_utils::down_cast;
    use crate::dynamic_connectivity::union_find::UnionFind;
    use crate::dynamic_connectivity::weighted_quick_union::WeightedQuickUnion;
    use easy_assert::bool_assertions::BooleanAssert;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::{actual_vec, expected_vec};

    #[test]
    fn init_correctly() {
        let union_find = WeightedQuickUnion::new(10);

        let quick_union = down_cast::<WeightedQuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
            .in_order();
    }

    #[test]
    fn adding_object_as_index_with_same_value() {
        let mut union_find = WeightedQuickUnion::new(5);
        // as index starting from 0, the next val would be 5
        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        let quick_find = down_cast::<WeightedQuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn adding_will_error_if_object_exist() {
        let mut union_find = WeightedQuickUnion::new(6);
        // as index starting from 0, the next val would be 6
        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_err()).is_true();

        let quick_find = down_cast::<WeightedQuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_happy_path() {
        let mut quick_union = WeightedQuickUnion {
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
        let mut quick_union = WeightedQuickUnion {
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
            .is_equal_to(expected_vec(vec![0, 0, 0, 0, 0, 0, 5, 5, 7, 9]))
            .in_order();
        ListAssert::assert_that(actual_vec(quick_union.sizes.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![9, 1, 1, 1, 1, 4, 1, 2, 1, 1]))
            .in_order();
    }

    #[test]
    fn union_first_object_is_missing() {
        let mut quick_union = WeightedQuickUnion {
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
        let mut quick_union = WeightedQuickUnion {
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
        let mut quick_union = WeightedQuickUnion {
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
        let mut quick_union = WeightedQuickUnion {
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
        let quick_union = WeightedQuickUnion {
            objects: vec![0, 1, 2, 3, 4],
            sizes: vec![1, 1, 1, 1, 1],
        };

        let result = quick_union.connected(&0, &1);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_values_are_same() {
        let quick_union = WeightedQuickUnion {
            objects: vec![0, 3, 3, 3, 4],
            sizes: vec![1, 3, 3, 3, 1],
        };

        let result = quick_union.connected(&1, &3);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_false_when_parents_are_different() {
        let quick_union = WeightedQuickUnion {
            objects: vec![2, 0, 2, 3, 4, 4, 5, 6],
            sizes: vec![3, 3, 3, 1, 4, 4, 4, 4],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_parents_are_same() {
        let quick_union = WeightedQuickUnion {
            objects: vec![2, 0, 4, 3, 4, 4, 5, 6],
            sizes: vec![7, 7, 7, 7, 7, 7, 7, 7],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_error_when_value_is_missing() {
        let quick_union = WeightedQuickUnion {
            objects: vec![0, 3, 3, 3, 4],
            sizes: vec![1, 3, 3, 3, 1],
        };

        let result = quick_union.connected(&1, &5);

        BooleanAssert::assert_that(result.is_err()).is_true();
    }
}
