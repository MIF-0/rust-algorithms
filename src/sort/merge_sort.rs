use std::fmt::Debug;

pub fn sort<T>(array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    if array.is_empty() || array.len() == 1 {
        return array;
    }
    //Usual implementation requires copy, because it is not possible to extract value from array, we converting it to optional,
    // so when we will take value from it, it will left None
    let array = array.into_iter().map(|val| Some(val)).collect();

    sort_no_copy(array)
        .into_iter()
        .map(|val| val.expect("None should not be present in the result array"))
        .collect()
}

fn sort_no_copy<T>(mut array: Vec<Option<T>>) -> Vec<Option<T>>
where
    T: Ord + Debug,
{
    if array.is_empty() || array.len() == 1 {
        return array;
    }
    let middle = array.len() / 2;
    let second = array.split_off(middle);
    let sorted_first_part = sort_no_copy(array);
    let sorted_second_part = sort_no_copy(second);

    merge_sorted(sorted_first_part, sorted_second_part)
}

fn merge_sorted<T>(
    mut first_array: Vec<Option<T>>,
    mut second_array: Vec<Option<T>>,
) -> Vec<Option<T>>
where
    T: Ord + Debug,
{
    let mut merged: Vec<Option<T>> = Vec::with_capacity(first_array.len() + second_array.len());
    let mut first_index: usize = 0;
    let mut second_index: usize = 0;

    while first_index < first_array.len() || second_index < second_array.len() {
        let first_value = first_array.get_mut(first_index);
        let second_value = second_array.get_mut(second_index);
        match (first_value, second_value) {
            (Some(first), Some(second)) => {
                if first.as_ref().unwrap().lt(second.as_ref().unwrap()) {
                    merged.push(first.take());
                    first_index += 1;
                } else {
                    merged.push(second.take());
                    second_index += 1;
                }
            }
            (Some(value), None) => {
                first_index += 1;
                merged.push(value.take());
            }
            (None, Some(value)) => {
                second_index += 1;
                merged.push(value.take());
            }
            (None, None) => break,
        }
    }

    merged
}

#[cfg(test)]
mod test {
    use crate::sort::merge_sort::sort;
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
