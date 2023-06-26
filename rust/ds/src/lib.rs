use std::{borrow::Borrow, cell::RefCell, rc::Rc};

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

pub struct Queue<T> {
    tail: Option<Rc<RefCell<Node<T>>>>,
    head: Option<Rc<RefCell<Node<T>>>>,
    pub length: u64,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            tail: None,
            head: None,
            length: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.length += 1;
        let node = Rc::new(RefCell::new(Node::new(value)));

        if let None = self.tail {
            self.tail = Some(Rc::clone(&node));
            self.head = Some(Rc::clone(&(node)));
        } else {
            if let Some(tail) = &self.tail {
                tail.borrow_mut().next = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }

    pub fn deque(&mut self) {}

    // pub fn peek(&self) -> Option<&T> {
    //     match &self.head {
    //         Some(head) => head.as_ref().,
    //         None => None,
    //     }
    // }
}
