use crate::sort::shuffle_sort::shuffle;
use std::fmt::Debug;

pub fn sort<T>(array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    let array = shuffle(array);

    let len = array.len();

    sort_part(array, 0, len - 1)
}

fn sort_part<T>(array: Vec<T>, low: usize, hi_included: usize) -> Vec<T>
where
    T: Ord + Debug,
{
    if hi_included <= low {
        return array;
    }

    let (mut array, elem_at_place) = partition(array, low, hi_included);
    if elem_at_place > 0 {
        array = sort_part(array, low, elem_at_place - 1);
    }

    sort_part(array, elem_at_place + 1, hi_included)
}
fn partition<T>(mut array: Vec<T>, from: usize, till_included: usize) -> (Vec<T>, usize)
where
    T: Ord + Debug,
{
    let mut lower_index: usize = from + 1;
    let mut greater_index: usize = till_included;
    loop {
        while array[lower_index].le(&array[from]) {
            lower_index += 1;
            if lower_index >= till_included {
                break;
            }
        }

        while array[greater_index].gt(&array[from]) {
            greater_index -= 1;
            if greater_index <= from {
                break;
            }
        }
        if lower_index > greater_index {
            break;
        }
        array.swap(lower_index, greater_index);
    }
    array.swap(from, greater_index);

    (array, greater_index)
}

#[cfg(test)]
mod test {
    use crate::sort::quick_sort::sort;
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
