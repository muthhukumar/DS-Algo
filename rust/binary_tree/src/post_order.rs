use std::{cell::RefCell, rc::Rc};

use crate::bt::BinaryNode;

pub fn walk(head: Option<Rc<RefCell<BinaryNode>>>, path: &mut Vec<i32>) {
    match head {
        None => {
            return;
        }
        Some(head) => {
            walk(head.borrow().left.to_owned(), path);
            walk(head.borrow().right.to_owned(), path);
            path.push(head.borrow().value);
        }
    }

    return;
}

pub fn post_order_search(head: BinaryNode) -> Vec<i32> {
    let mut path = vec![];

    walk(Some(Rc::new(RefCell::new(head))), &mut path);

    path
}

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use crate::bt::BinaryNode;

    use super::post_order_search;

    #[test]
    fn post_order_search_test() {
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

        assert_eq!(
            post_order_search(tree),
            vec![7, 5, 15, 10, 29, 45, 30, 100, 50, 20,]
        )
    }
}
