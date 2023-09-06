use std::fmt::Debug;

pub fn sort<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord + Debug,
{
    let mut step = compute_step(array.len());
    while step >= 1 {
        for i in step..array.len() {
            let mut j = i;
            while j >= step {
                let current = &array[j];
                let prev = &array[j - step];
                if current.lt(prev) {
                    array.swap(j, j - step);
                    j -= step;
                } else {
                    break;
                }
            }
        }
        step /= 3;
    }

    array
}

fn compute_step(array_size: usize) -> usize {
    let mut step = 1;
    while step < array_size / 3 {
        step = 3 * step + 1;
    }

    step
}

#[cfg(test)]
mod test {
    use crate::sort::shell_sort::sort;
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
