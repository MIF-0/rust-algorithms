use std::fmt::Debug;

pub fn sort<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    for current_index in 1..array.len() {
        move_to_ordered_position(&mut array, current_index);
    }

    array
}

fn move_to_ordered_position<T>(array: &mut [T], current_index: usize)
where
    T: Ord + Debug,
{
    let mut current_index = current_index;
    let mut previous_index = current_index - 1;
    loop {
        let current = &array[current_index];
        let previous = &array[previous_index];
        if current.lt(previous) {
            array.swap(current_index, previous_index);
            if previous_index == 0 {
                break;
            }
            previous_index -= 1;
            current_index -= 1;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sort::insertion_sort::sort;
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
