use crate::collection::stack::Stack;
use crate::collection::Collection;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type MultiRefNode<T> = RefCell<Rc<RefCell<Option<Node<T>>>>>;

pub struct LinkedList<T> {
    head: MultiRefNode<T>,
    tail: MultiRefNode<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: RefCell::new(Rc::new(RefCell::new(None))),
            tail: RefCell::new(Rc::new(RefCell::new(None))),
            size: 0,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let tmp = self.head.borrow();
        let current_cell = tmp.borrow();
        let current = current_cell.as_ref();
        if let Some(node) = current {
            return write!(f, "Size: {} \n head {}", self.size.clone(), node);
        }

        write!(f, "Empty")
    }
}

struct Node<T> {
    item: T,
    next: MultiRefNode<T>,
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let next_tmp_val = self.next.borrow();
        let next_cell_val = next_tmp_val.borrow();
        let next_val = next_cell_val.as_ref();
        if let Some(node) = next_val {
            return write!(f, "value {} -> next  {}", self.item, node);
        }

        write!(f, "value {} ->; next -> None", self.item)
    }
}

impl<T> Collection for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Stack<T> for LinkedList<T> {
    fn push(&mut self, value: T) {
        let new_node = Node {
            item: value,
            next: RefCell::new(Rc::new(RefCell::new(None))),
        };
        let counted_node = Rc::new(RefCell::new(Some(new_node)));
        if self.is_empty() {
            self.tail = RefCell::new(Rc::clone(&counted_node));
            self.head = RefCell::new(counted_node);
        } else {
            let old_tail = self.tail.replace(Rc::clone(&counted_node));
            let mut old_ref = old_tail.borrow_mut();
            let old_node = old_ref.as_mut();
            if let Some(node) = old_node {
                node.next.replace(counted_node);
            }
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::different_linked_list::first_attempt_linked_list::LinkedList;
    use crate::collection::stack::Stack;

    #[test]
    fn happy_path() {
        let mut linked_list: LinkedList<String> = LinkedList::new();

        linked_list.push("a".to_string());
        linked_list.push("b".to_string());
        linked_list.push("c".to_string());

        println!("SIZE: {}", linked_list.size);
        println!("{}", linked_list);
    }
}
