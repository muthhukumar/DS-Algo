use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct DLNode<T: Clone + PartialEq> {
    value: T,
    prev: Option<Weak<RefCell<DLNode<T>>>>,
    next: Option<Rc<RefCell<DLNode<T>>>>,
}

impl<T: Clone + PartialEq> DLNode<T> {
    pub fn new(value: T) -> Rc<RefCell<DLNode<T>>> {
        Rc::new(RefCell::new(DLNode {
            value,
            prev: None,
            next: None,
        }))
    }
}

pub struct DoublyLinkedList<T: Clone + PartialEq> {
    head: Option<Rc<RefCell<DLNode<T>>>>,
    tail: Option<Weak<RefCell<DLNode<T>>>>,
    length: usize,
}

impl<T: Clone + PartialEq> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        self.length += 1;
        let node = DLNode::new(value);

        match self.tail.take() {
            Some(tail) => {
                if let Some(tail) = tail.upgrade() {
                    node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                    tail.borrow_mut().next = Some(Rc::clone(&node));
                } else {
                    self.head = Some(Rc::clone(&node));
                }

                self.tail = Some(Rc::downgrade(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::downgrade(&node));
            }
        }
    }
    pub fn prepend(&mut self, value: T) {
        self.length += 1;

        let node = DLNode::new(value);

        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(Rc::clone(&head));
                head.borrow_mut().prev = Some(Rc::downgrade(&node));

                self.head = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::downgrade(&node));
            }
        }
    }
    pub fn insert_at(&mut self, idx: usize, item: T) {
        if idx == 0 {
            return self.prepend(item);
        } else if idx == self.length() {
            return self.append(item);
        }

        if let Some(curr) = self.get_at(idx) {
            self.length += 1;

            let node = DLNode::new(item);

            node.borrow_mut().next = Some(Rc::clone(&curr));

            curr.borrow()
                .prev
                .as_ref()
                .and_then(|prev| prev.upgrade())
                .map(|prev| {
                    node.borrow_mut().prev = Some(Rc::downgrade(&prev));
                    curr.borrow_mut().prev = Some(Rc::downgrade(&node));
                    prev.borrow_mut().next = Some(Rc::clone(&node));
                });
        }
    }
    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut idx = 0;

        let curr = {
            let mut curr = self.head.clone();

            while let Some(node) = curr {
                if value == node.borrow().value {
                    curr = Some(node);
                    break;
                };

                idx += 1;
                curr = node.borrow().next.clone();
            }

            curr
        };

        if idx > self.length {
            return None;
        }

        self.length -= 1;

        if idx == 0 {
            let head = self.head.clone();
            let value = head.as_ref().unwrap().borrow().value.clone();

            if let Some(next) = head.unwrap().borrow().next.clone() {
                self.head = Some(next);
            }

            return Some(value);
        } else if idx == self.length() {
            let tail = self.tail.clone().as_ref().unwrap().upgrade().unwrap();
            let value = tail.borrow().value.clone();

            tail.borrow().prev.as_ref().unwrap().upgrade().map(|prev| {
                self.tail = Some(Rc::downgrade(&prev));
            });

            return Some(value);
        }

        let curr_prev = curr.as_ref().unwrap().borrow().prev.clone();
        let curr_next = curr.as_ref().unwrap().borrow().next.clone();

        curr_prev.and_then(|prev| prev.upgrade()).map(|prev| {
            prev.borrow_mut().next = Some(curr_next.clone().unwrap().clone());
        });

        curr_next.and_then(|next| Some(next)).map(|next| {
            next.borrow_mut().prev = Some(Rc::downgrade(&next));
        });

        let value = curr.as_ref().unwrap().borrow().value.clone();

        Some(value)
    }
    pub fn remove_at(&mut self, idx: usize) -> Option<T> {
        if idx > self.length {
            return None;
        }

        self.length -= 1;

        if idx == 0 {
            let head = self.head.clone();
            let value = head.as_ref().unwrap().borrow().value.clone();

            if let Some(next) = head.unwrap().borrow().next.clone() {
                self.head = Some(next);
            }

            return Some(value);
        } else if idx == self.length() {
            let tail = self.tail.clone().as_ref().unwrap().upgrade().unwrap();
            let value = tail.borrow().value.clone();

            tail.borrow().prev.as_ref().unwrap().upgrade().map(|prev| {
                self.tail = Some(Rc::downgrade(&prev));
            });

            return Some(value);
        }
        let curr = self.get_at(idx);

        let curr_prev = curr.as_ref().unwrap().borrow().prev.clone();
        let curr_next = curr.as_ref().unwrap().borrow().next.clone();

        curr_prev.and_then(|prev| prev.upgrade()).map(|prev| {
            prev.borrow_mut().next = Some(curr_next.clone().unwrap().clone());
        });

        curr_next.and_then(|next| Some(next)).map(|next| {
            next.borrow_mut().prev = Some(Rc::downgrade(&next));
        });

        let value = curr.as_ref().unwrap().borrow().value.clone();

        Some(value)
    }
    pub fn get(&self, idx: usize) -> Option<T> {
        match self.get_at(idx) {
            Some(curr) => Some(curr.borrow().value.clone()),
            None => None,
        }
    }
    pub fn length(&self) -> usize {
        self.length
    }

    fn get_at(&self, idx: usize) -> Option<Rc<RefCell<DLNode<T>>>> {
        let mut current = self.head.clone();
        let mut curr_idx = 0;

        while let Some(node) = current {
            if curr_idx == idx {
                return Some(node);
            }

            current = node.borrow().next.clone();
            curr_idx += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::DoublyLinkedList;

    #[test]
    fn doubly_linked_list_test() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

        list.append(5);
        list.append(7);
        list.append(9);

        list.prepend(19);

        // list: 19, 5, 7, 9

        assert_eq!(list.get(0).unwrap(), 19);
        assert_eq!(list.length(), 4);

        list.insert_at(0, 100);

        // list: 100, 19, 5, 7, 9

        assert_eq!(list.remove(100).unwrap(), 100);

        // list: 19, 5, 7, 9

        assert_eq!(list.get(0).unwrap(), 19);
        assert_eq!(list.remove(9).unwrap(), 9);

        // list: 19, 5, 7

        assert_eq!(list.length(), 3);
        assert_eq!(list.get(2).unwrap(), 7);
        assert_eq!(list.get(0).unwrap(), 19);
        assert_eq!(list.get(1).unwrap(), 5);

        list.prepend(84);
        list.prepend(34);

        // list: 34, 84, 19, 5, 7
        assert_eq!(list.length(), 5);
        assert_eq!(list.get(0).unwrap(), 34);
        assert_eq!(list.get(1).unwrap(), 84);
        assert_eq!(list.get(2).unwrap(), 19);
        assert_eq!(list.get(3).unwrap(), 5);
        assert_eq!(list.get(4).unwrap(), 7);

        // list: 34, 84, 19, 5, 7
        assert_eq!(list.remove(19).unwrap(), 19);
        assert_eq!(list.length(), 4);
        assert_eq!(list.get(3).unwrap(), 7);

        // list: 34, 84, 5, 7

        assert_eq!(list.get(0).unwrap(), 34);
        assert_eq!(list.remove_at(7), None);

        // list: 34, 84, 5, 7
        //
        assert_eq!(list.remove_at(1).unwrap(), 84);

        // list: 34, 5, 7

        assert_eq!(list.length, 3);
        assert_eq!(list.remove_at(0).unwrap(), 34);
        assert_eq!(list.remove_at(1).unwrap(), 7);
        assert_eq!(list.remove_at(0).unwrap(), 5);
        assert_eq!(list.length, 0);

        // list: empty
    }
}
