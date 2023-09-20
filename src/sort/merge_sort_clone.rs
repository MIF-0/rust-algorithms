use std::fmt::Debug;

pub fn sort<T>(array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug + Clone,
{
    sort_slice(array.as_slice())
}

fn sort_slice<T>(array: &[T]) -> Vec<T>
where
    T: Ord + Debug + Clone,
{
    if array.is_empty() || array.len() == 1 {
        return array.to_vec();
    }
    let middle = array.len() / 2;
    let (first, second) = array.split_at(middle);
    let sorted_first_part = sort_slice(first);
    let sorted_second_part = sort_slice(second);

    merge_sorted(sorted_first_part, sorted_second_part)
}

fn merge_sorted<T>(first_array: Vec<T>, second_array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug + Clone,
{
    let mut merged = Vec::with_capacity(first_array.len() + second_array.len());
    let mut first_index: usize = 0;
    let mut second_index: usize = 0;

    while first_index < first_array.len() || second_index < second_array.len() {
        let first_value = first_array.get(first_index);
        let second_value = second_array.get(second_index);
        match (first_value, second_value) {
            (Some(first), Some(second)) => {
                if first.lt(second) {
                    merged.push(first.to_owned());
                    first_index += 1;
                } else {
                    merged.push(second.to_owned());
                    second_index += 1;
                }
            }
            (Some(value), None) => {
                first_index += 1;
                merged.push(value.to_owned());
            }
            (None, Some(value)) => {
                second_index += 1;
                merged.push(value.to_owned());
            }
            (None, None) => break,
        }
    }

    merged
}

#[cfg(test)]
mod test {
    use crate::sort::merge_sort_clone::sort;
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
