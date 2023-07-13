use crate::collection::dequeue::Dequeue;
use crate::collection::stack::Stack;
use crate::collection::Collection;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    size: usize,
}

struct Node<T> {
    value: T,
    front: Link<T>,
    back: Link<T>,
}

impl<T> Node<T> {
    pub fn new_front(value: T, back: Link<T>) -> Self {
        Node {
            value,
            front: None,
            back,
        }
    }

    pub fn new_back(value: T, front: Link<T>) -> Self {
        Node {
            value,
            front,
            back: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            size: 0,
        }
    }

    fn init_with_first_value(&mut self, value: T) {
        let new_front = Rc::new(RefCell::new(Node {
            value,
            front: None,
            back: None,
        }));
        self.front = Some(new_front.clone());
        self.back = Some(new_front);
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
        self.push_front(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }
}

impl<T> Dequeue<T> for LinkedList<T> {
    fn push_front(&mut self, value: T) {
        if let Some(old_front) = self.front.take() {
            let new_front = Rc::new(RefCell::new(Node::new_front(
                value,
                Some(old_front.clone()),
            )));
            old_front.borrow_mut().front = Some(new_front.clone());
            self.front = Some(new_front);
        } else {
            self.init_with_first_value(value);
        }
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            match node.borrow_mut().back.take() {
                None => {
                    self.back.take();
                }
                Some(new_front) => {
                    new_front.borrow_mut().front.take();
                    self.front = Some(new_front);
                }
            };
            self.size -= 1;
            Rc::try_unwrap(node).ok().unwrap().into_inner().value
        })
    }

    fn push_back(&mut self, value: T) {
        if let Some(old_back) = self.back.take() {
            let new_back = Rc::new(RefCell::new(Node::new_back(value, Some(old_back.clone()))));
            old_back.borrow_mut().back = Some(new_back.clone());
            self.back = Some(new_back);
        } else {
            self.init_with_first_value(value);
        }
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|node| {
            match node.borrow_mut().front.take() {
                None => {
                    self.front.take();
                }
                Some(new_back) => {
                    new_back.borrow_mut().back.take();
                    self.back = Some(new_back);
                }
            };
            self.size -= 1;
            Rc::try_unwrap(node).ok().unwrap().into_inner().value
        })
    }
}

#[cfg(test)]
mod test {
    use crate::collection::dequeue::Dequeue;
    use crate::collection::simple_linked_list::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
