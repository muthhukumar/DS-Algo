use std::{cell::RefCell, fmt::Debug as TDebug, rc::Rc};

#[derive(Debug)]
struct DLNode<T: Clone + PartialEq + TDebug> {
    value: T,
    prev: Option<Rc<RefCell<DLNode<T>>>>,
    next: Option<Rc<RefCell<DLNode<T>>>>,
}

impl<T: Clone + PartialEq + TDebug> DLNode<T> {
    pub fn new(value: T) -> DLNode<T> {
        DLNode {
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct DoublyLinkedList<T: Clone + PartialEq + TDebug> {
    head: Option<Rc<RefCell<DLNode<T>>>>,
    tail: Option<Rc<RefCell<DLNode<T>>>>,
    length: usize,
}

impl<T: Clone + PartialEq + TDebug> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        self.length += 1;
        let node = Rc::new(RefCell::new(DLNode::new(value)));

        match self.tail.take() {
            Some(tail) => {
                node.borrow_mut().prev = Some(Rc::clone(&tail));
                tail.borrow_mut().next = Some(Rc::clone(&node));

                self.tail = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }
    pub fn prepend(&mut self, value: T) {
        self.length += 1;

        let node = Rc::new(RefCell::new(DLNode::new(value)));

        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(Rc::clone(&head));
                head.borrow_mut().prev = Some(Rc::clone(&node));

                self.head = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }
    pub fn insert_at(&mut self, item: T, idx: usize) {
        if idx == 0 {
            return self.prepend(item);
        } else if idx == self.length {
            return self.append(item);
        }

        match self.get_at(idx) {
            Some(curr) => {
                self.length += 1;

                let node = Rc::new(RefCell::new(DLNode::new(item)));

                node.borrow_mut().next = Some(Rc::clone(&curr));

                if let Some(curr_prev) = curr.borrow().prev.clone() {
                    node.borrow_mut().prev = Some(Rc::clone(&curr_prev));
                    curr_prev.borrow_mut().next = Some(Rc::clone(&node));
                }

                curr.borrow_mut().prev = Some(Rc::clone(&node));
            }
            None => {}
        }
    }
    pub fn remove(&mut self, value: T) -> Option<T> {
        let curr = match &self.head {
            Some(head) => {
                let mut curr = head.clone();

                for _ in 0..self.length {
                    if curr.borrow().value == value {
                        break;
                    }
                    if let Some(next) = curr.clone().borrow().next.clone() {
                        curr = next;
                    }
                }

                Some(curr)
            }
            None => return None,
        };

        match curr {
            Some(curr) => {
                self.length -= 1;

                if let Some(head) = &self.head {
                    if Rc::ptr_eq(head, &curr) {
                        if let Some(head) = self.head.take() {
                            self.head = head.borrow().next.clone();

                            head.borrow_mut().next = None;
                            head.borrow_mut().prev = None;

                            return Some(head.borrow().value.clone());
                        }
                    }
                }
                if let Some(tail) = &self.tail {
                    if Rc::ptr_eq(tail, &curr) {
                        if let Some(tail) = self.tail.take() {
                            self.tail = tail.borrow().prev.clone();

                            tail.borrow_mut().next = None;
                            tail.borrow_mut().prev = None;

                            return Some(tail.borrow().value.clone());
                        }
                    }
                }

                let prev = curr.borrow().prev.clone();
                let next = curr.borrow().next.clone();

                match (&prev, &next) {
                    (Some(curr_prev), Some(curr_next)) => {
                        curr_prev.borrow_mut().next = Some(Rc::clone(&curr_next));
                    }
                    _ => {}
                }

                match (&prev, &next) {
                    (Some(curr_prev), Some(curr_next)) => {
                        curr_next.borrow_mut().prev = Some(Rc::clone(&curr_prev));
                    }
                    _ => {}
                }

                Some(curr.borrow().value.clone())
            }
            None => None,
        }
    }
    pub fn remove_at(&mut self, idx: usize) -> Option<T> {
        match self.get_at(idx) {
            Some(curr) => {
                self.length -= 1;

                if let Some(head) = &self.head {
                    if Rc::ptr_eq(head, &curr) {
                        if let Some(head) = self.head.take() {
                            self.head = head.borrow().next.clone();

                            head.borrow_mut().next = None;
                            head.borrow_mut().prev = None;

                            return Some(head.borrow().value.clone());
                        }
                    }
                }
                if let Some(tail) = &self.tail {
                    // println!("tail: {:?}", tail);
                    // println!("curr: {:?}", curr);
                    if Rc::ptr_eq(tail, &curr) {
                        println!("it should come to this part");
                        if let Some(tail) = self.tail.take() {
                            self.tail = tail.borrow().prev.clone();

                            tail.borrow_mut().next = None;
                            tail.borrow_mut().prev = None;

                            return Some(tail.borrow().value.clone());
                        }
                    }
                }

                let prev = curr.borrow().prev.clone();
                let next = curr.borrow().next.clone();

                match (&prev, &next) {
                    (Some(curr_prev), Some(curr_next)) => {
                        curr_prev.borrow_mut().next = Some(Rc::clone(&curr_next));
                    }
                    _ => {}
                }

                match (&prev, &next) {
                    (Some(curr_prev), Some(curr_next)) => {
                        curr_next.borrow_mut().prev = Some(Rc::clone(&curr_prev));
                    }
                    _ => {}
                }

                Some(curr.borrow().value.clone())
            }
            None => {
                println!("It came to his None");
                return None;
            }
        };

        None
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
        match &self.head {
            Some(head) => {
                let mut curr = head.clone();

                for _ in 0..idx {
                    if let Some(next) = curr.clone().borrow().next.clone() {
                        curr = next;
                    }
                }

                Some(curr)
            }
            None => None,
        }
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

        println!("size: {}", list.length);
        assert_eq!(list.get(2).unwrap(), 9);
        println!("get at 2: {}", list.get(2).unwrap());
        assert_eq!(list.remove_at(1).unwrap(), 7);
        assert_eq!(list.length, 2);

        list.append(11);
        assert_eq!(list.remove_at(1).unwrap(), 9);
        assert_eq!(list.remove(9).unwrap(), 9);
        assert_eq!(list.remove_at(0).unwrap(), 5);
        assert_eq!(list.remove_at(0).unwrap(), 11);
        assert_eq!(list.length, 0);

        list.prepend(5);
        list.prepend(7);
        list.prepend(9);

        assert_eq!(list.get(2).unwrap(), 5);
        assert_eq!(list.get(0).unwrap(), 9);
        assert_eq!(list.remove(9).unwrap(), 9);
        assert_eq!(list.length, 2);
        assert_eq!(list.get(0).unwrap(), 7);
    }
}
