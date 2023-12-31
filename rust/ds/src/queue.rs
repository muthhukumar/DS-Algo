use std::{cell::RefCell, rc::Rc};

pub struct QNode<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<QNode<T>>>>,
}

impl<T: Clone> QNode<T> {
    pub fn new(value: T) -> Rc<RefCell<QNode<T>>> {
        Rc::new(RefCell::new(QNode { value, next: None }))
    }
}

pub struct Queue<T: Clone> {
    head: Option<Rc<RefCell<QNode<T>>>>,
    tail: Option<Rc<RefCell<QNode<T>>>>,
    size: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.size += 1;

        let node = QNode::new(value);

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                if let Some(next) = head.borrow_mut().next.take() {
                    self.head = Some(Rc::clone(&next));
                } else {
                    self.head = None;
                    self.tail = None;
                }

                self.size -= 1;
                Some(head.borrow().value.clone())
            }
            None => {
                self.tail = None;
                None
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Some(head) => Some(head.borrow().value.clone()),
            None => None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn should_have_all_queued_elements() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(3, queue.size());

        assert_eq!(1, queue.dequeue().unwrap());
        assert_eq!(2, queue.dequeue().unwrap());
        assert_eq!(3, queue.dequeue().unwrap());

        assert_eq!(0, queue.size());
    }
}
