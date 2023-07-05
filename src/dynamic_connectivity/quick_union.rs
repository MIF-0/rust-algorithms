use crate::dynamic_connectivity::union_find::UnionFind;
use crate::AlgoError;
use std::fmt;
use std::fmt::Display;

pub struct QuickUnion {
    objects: Vec<usize>,
}

impl QuickUnion {
    pub fn new(size: usize) -> Self {
        let mut objects: Vec<usize> = Vec::with_capacity(size);
        for i in 0..size {
            objects.push(i);
        }

        QuickUnion { objects }
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

impl UnionFind for QuickUnion {
    fn add(&mut self, object: usize) -> Result<(), AlgoError> {
        if self.objects.get(object).is_some() {
            return Err(AlgoError::element_already_exist("object", &object));
        }

        self.objects.insert(object, object);

        Ok(())
    }

    fn union(&mut self, first: &usize, second: &usize) -> Result<(), AlgoError> {
        let first_root = self.find_root(first);
        let second_root = self.find_root(second);

        match (first_root, second_root) {
            (Some(f), Some(s)) => {
                self.objects[f] = s;

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

impl Display for QuickUnion {
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
    use crate::dynamic_connectivity::quick_union::QuickUnion;
    use crate::dynamic_connectivity::test_utils::down_cast;
    use crate::dynamic_connectivity::union_find::UnionFind;
    use easy_assert::bool_assertions::BooleanAssert;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::{actual_vec, expected_vec};

    #[test]
    fn init_correctly() {
        let union_find = QuickUnion::new(10);

        let quick_union = down_cast::<QuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
            .in_order();
    }

    #[test]
    fn adding_object_as_index_with_same_value() {
        let mut union_find = QuickUnion::new(5);
        // as index starting from 0, the next val would be 5
        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        let quick_find = down_cast::<QuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn adding_will_error_if_object_exist() {
        let mut union_find = QuickUnion::new(6);
        // as index starting from 0, the next val would be 6
        let result = union_find.add(5);

        BooleanAssert::assert_that(result.is_err()).is_true();

        let quick_find = down_cast::<QuickUnion>(&union_find);
        ListAssert::assert_that(actual_vec(quick_find.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_happy_path() {
        let mut quick_union = QuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
        };
        let result = quick_union.union(&0, &3);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![3, 1, 2, 3, 4, 5]))
            .in_order();
    }

    #[test]
    fn union_first_object_is_missing() {
        let mut quick_union = QuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
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
        let mut quick_union = QuickUnion {
            objects: vec![0, 1, 2, 3, 4, 5],
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
        let mut quick_union = QuickUnion {
            objects: vec![0, 1, 2, 2, 2, 5, 6, 6, 6, 8, 9],
        };

        let result = quick_union.union(&2, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        ListAssert::assert_that(actual_vec(quick_union.objects.clone()))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 6, 2, 2, 5, 6, 6, 6, 8, 9]))
            .in_order();
    }

    #[test]
    fn connected_will_return_false_when_values_are_different() {
        let quick_union = QuickUnion {
            objects: vec![0, 1, 2, 3, 4],
        };

        let result = quick_union.connected(&0, &1);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_values_are_same() {
        let quick_union = QuickUnion {
            objects: vec![0, 3, 3, 3, 4],
        };

        let result = quick_union.connected(&1, &3);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_false_when_parents_are_different() {
        let quick_union = QuickUnion {
            objects: vec![2, 0, 2, 3, 4, 4, 5, 6],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_false()
    }

    #[test]
    fn connected_will_return_true_when_parents_are_same() {
        let quick_union = QuickUnion {
            objects: vec![2, 0, 4, 3, 4, 4, 5, 6],
        };

        let result = quick_union.connected(&1, &7);

        BooleanAssert::assert_that(result.is_ok()).is_true();
        BooleanAssert::assert_that(result.ok().expect("value")).is_true()
    }

    #[test]
    fn connected_will_return_error_when_value_is_missing() {
        let quick_union = QuickUnion {
            objects: vec![0, 3, 3, 3, 4],
        };

        let result = quick_union.connected(&1, &5);

        BooleanAssert::assert_that(result.is_err()).is_true();
    }
}
