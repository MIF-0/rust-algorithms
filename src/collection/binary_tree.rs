use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct BinarySearchTree<K, V>
where
    K: Ord,
{
    root: Link<K, V>,
    length: usize,
    _boo_key: PhantomData<K>,
    _boo_value: PhantomData<V>,
}

struct Node<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

struct DeleteResult<K, V>
where
    K: Ord,
{
    new_link: Link<K, V>,
    deleted_key: K,
    deleted_value: V,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

type Link<K, V> = Option<NonNull<Node<K, V>>>;

impl<K, V> Default for BinarySearchTree<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> BinarySearchTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
            _boo_key: PhantomData,
            _boo_value: PhantomData,
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        unsafe {
            let mut current = self.root.map(|node| &(*node.as_ptr()));
            loop {
                match current {
                    None => {
                        break;
                    }
                    Some(link) => {
                        let compare_result = key.cmp(&link.key);
                        match compare_result {
                            Ordering::Less => current = link.left.map(|node| &(*node.as_ptr())),
                            Ordering::Equal => {
                                return Some(&link.value);
                            }
                            Ordering::Greater => current = link.right.map(|node| &(*node.as_ptr())),
                        }
                    }
                }
            }
        }
        None
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = BinarySearchTree::upsert(self.root, key, value);
        self.length += 1;
    }

    pub fn min(&self) -> Option<&V> {
        return match self.root {
            None => None,
            Some(root_link) => unsafe {
                let current = Self::find_min(root_link);
                Some(&current.value)
            },
        };
    }

    pub fn max(&self) -> Option<&V> {
        unsafe {
            match self.root {
                None => None,
                Some(root_link) => {
                    let mut current = &(*root_link.as_ptr());
                    while current.right.is_some() {
                        match current.right {
                            None => {
                                break;
                            }
                            Some(link) => {
                                current = &(*link.as_ptr());
                            }
                        }
                    }
                    Some(&current.value)
                }
            }
        }
    }

    pub fn delete_min(&mut self) {
        match self.root {
            None => {
                //do nothing
            }
            Some(root) => unsafe {
                let result = Self::_delete_min(root);
                self.root = result.new_link;
            },
        }
    }

    pub fn delete(&mut self, key: K) {
        match self.root {
            None => {
                //do nothing
            }
            Some(root) => {
                unsafe {
                    let result = Self::_delete(root, key);
                    match result {
                        None => {
                            //do nothing
                        }
                        Some(value) => {
                            self.root = value.new_link;
                        }
                    }
                }
            }
        }
    }
    unsafe fn find_min<'a>(root_link: NonNull<Node<K, V>>) -> &'a Node<K, V> {
        let mut current = root_link.as_ref();
        while current.left.is_some() {
            match current.left {
                None => {
                    break;
                }
                Some(link) => {
                    current = link.as_ref();
                }
            }
        }
        current
    }

    unsafe fn _delete(root: NonNull<Node<K, V>>, key: K) -> Option<DeleteResult<K, V>> {
        let root_ptr = root.as_ptr();
        let compare_result = key.cmp(&(*root_ptr).key);
        match compare_result {
            Ordering::Less => {
                let left = (*root_ptr).left?;
                let result = Self::_delete(left, key)?;
                (*root_ptr).left = result.new_link;
                Some(DeleteResult {
                    new_link: Some(root),
                    deleted_key: result.deleted_key,
                    deleted_value: result.deleted_value,
                })
            }
            Ordering::Greater => {
                let right = (*root_ptr).right?;
                let result = Self::_delete(right, key)?;
                (*root_ptr).right = result.new_link;
                Some(DeleteResult {
                    new_link: Some(root),
                    deleted_key: result.deleted_key,
                    deleted_value: result.deleted_value,
                })
            }
            Ordering::Equal => {
                match ((*root_ptr).left, (*root_ptr).right) {
                    (Some(_), Some(right)) => {
                        // find a min in the right sub tree
                        let deleted_min_result = Self::_delete_min(right);
                        let deleted_key =
                            mem::replace(&mut (*root_ptr).key, deleted_min_result.deleted_key);
                        let deleted_value =
                            mem::replace(&mut (*root_ptr).value, deleted_min_result.deleted_value);
                        let final_result = DeleteResult {
                            new_link: Some(root),
                            deleted_key,
                            deleted_value,
                        };
                        Some(final_result)
                    }
                    (None, Some(_)) => {
                        let v = Box::from_raw(root_ptr);
                        Some(DeleteResult {
                            new_link: v.right,
                            deleted_key: v.key,
                            deleted_value: v.value,
                        })
                    }
                    (Some(_), None) => {
                        let v = Box::from_raw(root_ptr);
                        Some(DeleteResult {
                            new_link: v.left,
                            deleted_key: v.key,
                            deleted_value: v.value,
                        })
                    }
                    (None, None) => {
                        let v = Box::from_raw(root_ptr);
                        Some(DeleteResult {
                            new_link: None,
                            deleted_key: v.key,
                            deleted_value: v.value,
                        })
                    }
                }
            }
        }
    }

    unsafe fn _delete_min(root: NonNull<Node<K, V>>) -> DeleteResult<K, V> {
        let root_ref = root.as_ptr();

        match (*root_ref).left {
            None => {
                let v = Box::from_raw(root_ref);
                DeleteResult {
                    new_link: v.right,
                    deleted_key: v.key,
                    deleted_value: v.value,
                }
            }
            Some(left) => {
                let result = Self::_delete_min(left);
                (*root_ref).left = result.new_link;
                DeleteResult {
                    new_link: Some(root),
                    deleted_key: result.deleted_key,
                    deleted_value: result.deleted_value,
                }
            }
        }
    }

    pub fn clean(&mut self) {
        BinarySearchTree::remove_tree(self.root.take())
    }

    fn remove_tree(root: Link<K, V>) {
        if root.is_none() {
            return;
        }
        let left_node;
        let right_node;
        unsafe {
            let mut node_to_drop = root
                .map(|node| Box::from_raw(node.as_ptr()))
                .expect("some value");
            left_node = node_to_drop.left.take();
            right_node = node_to_drop.right.take();
        }
        BinarySearchTree::remove_tree(left_node);
        BinarySearchTree::remove_tree(right_node);
    }

    fn upsert(node_link: Link<K, V>, key: K, value: V) -> Link<K, V> {
        match node_link {
            None => {
                let new_node = Node::new(key, value);
                let new_box = Box::new(new_node);
                let ptr = Box::into_raw(new_box);
                unsafe {
                    let result = NonNull::new_unchecked(ptr);
                    Some(result)
                }
            }
            Some(node) => {
                unsafe {
                    let ptr_node = node.as_ptr();
                    let compare_result = key.cmp(&(*ptr_node).key);
                    match compare_result {
                        Ordering::Less => {
                            (*ptr_node).left =
                                BinarySearchTree::upsert((*ptr_node).left, key, value);
                        }
                        Ordering::Equal => {
                            (*ptr_node).value = value;
                        }
                        Ordering::Greater => {
                            (*ptr_node).right =
                                BinarySearchTree::upsert((*ptr_node).right, key, value);
                        }
                    };
                }
                node_link
            }
        }
    }
}

impl<K, V> Drop for BinarySearchTree<K, V>
where
    K: Ord,
{
    fn drop(&mut self) {
        self.clean();
    }
}

#[cfg(test)]
mod test {
    use crate::collection::binary_tree::BinarySearchTree;
    use easy_assert::num_assertions::NumericAssert;
    use easy_assert::option_assertions::OptionAssert;
    use easy_assert::string_assertions::StringAssert;
    use easy_assert::{actual, expected};

    #[test]
    fn basics_push() {
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");
        tree.put(12, "U");

        NumericAssert::assert_that(actual(tree.length))
            .is_equal()
            .to(expected(5));
        unsafe {
            let root = tree.root.expect("not null").as_ref();
            NumericAssert::assert_that(actual(root.key.clone()))
                .is_equal()
                .to(expected(10));

            StringAssert::assert_that(actual(root.value.to_string()))
                .is_equal()
                .to(expected("T".to_string()));

            let root_left_child = root.left.expect("not null").as_ref();
            NumericAssert::assert_that(actual(root_left_child.key.clone()))
                .is_equal()
                .to(expected(4));

            StringAssert::assert_that(actual(root_left_child.value.to_string()))
                .is_equal()
                .to(expected("D".to_string()));

            let root_right_child = root.right.expect("not null").as_ref();
            NumericAssert::assert_that(actual(root_right_child.key.clone()))
                .is_equal()
                .to(expected(12));

            StringAssert::assert_that(actual(root_right_child.value.to_string()))
                .is_equal()
                .to(expected("U".to_string()));

            let child_left_child = root_left_child.left.expect("not null").as_ref();
            NumericAssert::assert_that(actual(child_left_child.key.clone()))
                .is_equal()
                .to(expected(1));

            StringAssert::assert_that(actual(child_left_child.value.to_string()))
                .is_equal()
                .to(expected("A".to_string()));

            let child_right_child = root_left_child.right.expect("not null").as_ref();
            NumericAssert::assert_that(actual(child_right_child.key.clone()))
                .is_equal()
                .to(expected(7));

            StringAssert::assert_that(actual(child_right_child.value.to_string()))
                .is_equal()
                .to(expected("K".to_string()));
        }
    }

    #[test]
    fn basics_get() {
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");

        let result = tree.get(7).copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("K".to_string()));
    }

    #[test]
    fn basics_min() {
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");

        let result = tree.min().copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("A".to_string()));
    }

    #[test]
    fn basics_max() {
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");

        let result = tree.max().copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("T".to_string()));
    }

    #[test]
    fn basics_delete_min() {
        // GIVEN
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");

        let result = tree.min().copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("A".to_string()));

        // WHEN
        tree.delete_min();

        // THEN
        let result = tree.min().copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("D".to_string()));
    }

    #[test]
    fn basics_delete() {
        // GIVEN
        let mut tree: BinarySearchTree<usize, &str> = BinarySearchTree::new();
        tree.put(10, "T");
        tree.put(4, "D");
        tree.put(1, "A");
        tree.put(7, "K");

        let result = tree.get(7).copied().unwrap_or("IT IS EMPTY");

        StringAssert::assert_that(actual(result.to_string()))
            .is_equal()
            .to(expected("K".to_string()));

        // WHEN
        tree.delete(7);

        // THEN
        let result = tree.get(7).copied();

        OptionAssert::assert_that(result).is_none();
    }
}
