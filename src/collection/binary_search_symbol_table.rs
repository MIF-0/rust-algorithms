use std::cmp::Ordering;
use std::fmt::Debug;

pub struct SymbolTable<Key, Value>
where
    Key: Ord + Debug,
{
    keys: Vec<Key>,
    values: Vec<Value>,
}

impl<Key, Value> SymbolTable<Key, Value>
where
    Key: Ord + Debug,
{
    pub fn new() -> SymbolTable<Key, Value> {
        SymbolTable {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn put(&mut self, key: Key, value: Value) {
        if self.keys.is_empty() {
            self.keys.push(key);
            self.values.push(value);
            return;
        }

        let (current_index, needed_index) = self.index(&key);
        if let Some(current_index) = current_index {
            self.values[current_index] = value;
        } else {
            self.keys.insert(needed_index, key);
            self.values.insert(needed_index, value);
        }
    }

    pub fn get(&self, key: Key) -> Option<&Value> {
        let (current_index, _) = self.index(&key);
        return if let Some(current_index) = current_index {
            self.values.get(current_index)
        } else {
            None
        };
    }

    fn index(&self, key: &Key) -> (Option<usize>, usize) {
        let mut low: usize = 0;
        let mut high: usize = self.keys.len();
        loop {
            let mid = (low + high) / 2;
            let mid_value = self.keys.get(mid);
            let compare_result = mid_value.expect("unexpected empty value").cmp(key);
            match compare_result {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal => {
                    return (Some(mid), mid);
                }
                Ordering::Greater => {
                    high = mid;
                }
            }

            if low >= high {
                break;
            }
        }

        (None, low)
    }
}

impl<Key, Value> Default for SymbolTable<Key, Value>
where
    Key: Ord + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::collection::binary_search_symbol_table::SymbolTable;
    use easy_assert::list_assertions::ListAssert;
    use easy_assert::string_assertions::StringAssert;
    use easy_assert::{actual, actual_vec, expected, expected_vec};

    #[test]
    fn basics_push() {
        let mut symbol_table: SymbolTable<usize, &str> = SymbolTable::new();
        symbol_table.put(10, "T");
        symbol_table.put(4, "D");
        symbol_table.put(1, "A");
        symbol_table.put(7, "K");

        ListAssert::assert_that(actual_vec(symbol_table.keys))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec![1, 4, 7, 10]))
            .in_order();
        ListAssert::assert_that(actual_vec(symbol_table.values))
            .with_element_matcher(|a, b| a.eq(b))
            .is_equal_to(expected_vec(vec!["A", "D", "K", "T"]))
            .in_order();
    }

    #[test]
    fn basics_get() {
        let mut symbol_table: SymbolTable<usize, &str> = SymbolTable::new();
        symbol_table.put(10, "T");
        symbol_table.put(4, "D");
        symbol_table.put(1, "A");
        symbol_table.put(7, "K");

        let result = symbol_table.get(7).copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("K".to_string()));
    }
}
