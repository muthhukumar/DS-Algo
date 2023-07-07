use crate::bt::BinaryNode;
use ds::queue::Queue;
use std::{cell::RefCell, rc::Rc};

pub fn bfs(head: BinaryNode, needle: i32) -> bool {
    let mut queue: Queue<Rc<RefCell<BinaryNode>>> = Queue::new();

    queue.enqueue(Rc::new(RefCell::new(head)));

    while queue.size() > 0 {
        match queue.dequeue() {
            None => continue,
            Some(head) => {
                let head = head.borrow();

                if head.value == needle {
                    return true;
                }

                if let Some(right) = head.right.to_owned() {
                    queue.enqueue(right);
                }

                if let Some(left) = head.left.to_owned() {
                    queue.enqueue(left);
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use crate::bt::BinaryNode;

    use super::bfs;

    #[test]
    fn bfs_find() {
        let tree = BinaryNode {
            value: 20,
            right: Some(Rc::new(RefCell::new(BinaryNode {
                value: 50,
                right: Some(Rc::new(RefCell::new(BinaryNode {
                    value: 100,
                    left: None,
                    right: None,
                }))),
                left: Some(Rc::new(RefCell::new(BinaryNode {
                    value: 30,
                    left: Some(Rc::new(RefCell::new(BinaryNode {
                        value: 29,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(BinaryNode {
                        value: 45,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            left: Some(Rc::new(RefCell::new(BinaryNode {
                value: 10,
                left: Some(Rc::new(RefCell::new(BinaryNode {
                    value: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(BinaryNode {
                        value: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(BinaryNode {
                    value: 15,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        assert!(bfs(tree.clone(), 45));
        assert!(bfs(tree.clone(), 7));
        assert!(!bfs(tree.clone(), 69));
    }
}
