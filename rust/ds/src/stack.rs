use std::{cell::RefCell, rc::Rc};

struct SNode {
    value: i32,
    prev: Option<Rc<RefCell<SNode>>>,
}

impl SNode {
    pub fn new(value: i32) -> Rc<RefCell<SNode>> {
        Rc::new(RefCell::new(SNode { value, prev: None }))
    }
}

pub struct Stack {
    head: Option<Rc<RefCell<SNode>>>,
    size: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: i32) {
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

    pub fn pop(&mut self) -> Option<i32> {
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
                Some(head.borrow().value)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(head) => Some(head.borrow().value),
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
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(2, stack.size());
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.pop().unwrap());

        stack.push(1);

        assert_eq!(1, stack.size());
    }
}
