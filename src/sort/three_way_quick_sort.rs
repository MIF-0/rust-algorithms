use crate::sort::shuffle_sort::shuffle;
use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::fmt::Debug;
use Ordering::{Equal, Greater};

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

    let (mut array, lower_at_place, greater_at_place) = partition(array, low, hi_included);
    if lower_at_place > 0 {
        array = sort_part(array, low, lower_at_place - 1);
    }

    sort_part(array, greater_at_place + 1, hi_included)
}
fn partition<T>(mut array: Vec<T>, from: usize, till_included: usize) -> (Vec<T>, usize, usize)
where
    T: Ord + Debug,
{
    let mut lower_index: usize = from;
    let mut greater_index: usize = till_included;
    let mut current_index: usize = lower_index;
    loop {
        match array[current_index].cmp(&array[lower_index]) {
            Less => {
                array.swap(current_index, lower_index);
                lower_index += 1;
                current_index += 1;
            }
            Greater => {
                array.swap(current_index, greater_index);
                greater_index -= 1;
            }
            Equal => {
                current_index += 1;
            }
        }
        if current_index > greater_index {
            break;
        }
    }

    (array, lower_index, greater_index)
}

#[cfg(test)]
mod test {
    use crate::sort::three_way_quick_sort::sort;
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
