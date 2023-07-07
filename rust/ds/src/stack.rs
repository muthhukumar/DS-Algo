use std::{cell::RefCell, rc::Rc};

struct SNode<T: Clone> {
    value: T,
    prev: Option<Rc<RefCell<SNode<T>>>>,
}

impl<T: Clone> SNode<T> {
    pub fn new(value: T) -> Rc<RefCell<SNode<T>>> {
        Rc::new(RefCell::new(SNode { value, prev: None }))
    }
}

pub struct Stack<T: Clone> {
    head: Option<Rc<RefCell<SNode<T>>>>,
    size: usize,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = SNode::new(value);

        match self.head.take() {
            Some(head) => {
                node.borrow_mut().prev = Some(Rc::clone(&head));
                self.head = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
            }
        }

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                match head.borrow_mut().prev.take() {
                    Some(prev) => {
                        self.head = Some(Rc::clone(&prev));
                    }
                    None => {
                        self.head = None;
                    }
                }

                self.size -= 1;
                Some(head.borrow().value.clone())
            }
            None => None,
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
    use super::Stack;

    #[test]
    fn should_have_pushed_elements() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(2, stack.size());
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.pop().unwrap());

        stack.push(1);

        assert_eq!(1, stack.size());
    }
}
