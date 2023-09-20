use rand::Rng;
use std::fmt::Debug;

pub fn shuffle<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    let mut rng = rand::thread_rng();
    for index in 0..array.len() {
        let rand_to = index + 1;
        let rand = rng.gen_range(0..rand_to);
        array.swap(index, rand);
    }

    array
}

#[cfg(test)]
mod test {
    use crate::sort::shuffle_sort::shuffle;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::{actual_vec, expected_vec};

    #[test]
    fn basics() {
        let array = vec![3, 2, 3, 4, 1, 6, 9, 0];

        let sorted = shuffle(array);

        ListAssert::assert_that(actual_vec(sorted))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 3, 4, 6, 9]))
            .in_any_order();
    }

    #[test]
    fn order_is_different() {
        let array = vec![3, 2, 3, 4, 1, 6, 9, 0];

        let sorted = shuffle(array);

        ListAssert::assert_that(actual_vec(sorted))
            .with_element_matcher(|a, b| a.eq(b))
            .is_not_equal_to(expected_vec(vec![3, 2, 3, 4, 1, 6, 9, 0]))
            .in_order();
    }
}
