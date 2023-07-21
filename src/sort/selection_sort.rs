pub fn sort<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    for index in 0..array.len() {
        let min_index = find_smallest_from(&mut array, index);
        array.swap(index, min_index);
    }

    array
}

fn find_smallest_from<T>(array: &mut [T], index: usize) -> usize
where
    T: Ord,
{
    let mut min_index = index;
    let start = index + 1;
    for (current_index, current_val) in array.iter().enumerate().skip(start) {
        if current_val.lt(&array[min_index]) {
            min_index = current_index;
        }
    }
    min_index
}

#[cfg(test)]
mod test {
    use crate::sort::selection_sort::sort;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::{actual_vec, expected_vec};

    #[test]
    fn basics() {
        let array = vec![3, 2, 3, 4, 1, 6, 9, 0];

        let sorted = sort(array);

        ListAssert::assert_that(actual_vec(sorted))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![0, 1, 2, 3, 3, 4, 6, 9]))
            .in_order();
    }
}
