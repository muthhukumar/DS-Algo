use std::{cell::RefCell, rc::Rc};

pub struct BinaryNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<BinaryNode>>>,
    pub right: Option<Rc<RefCell<BinaryNode>>>,
}

impl BinaryNode {
    pub fn new(value: i32) -> BinaryNode {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl Clone for BinaryNode {
    fn clone(&self) -> Self {
        BinaryNode {
            value: self.value,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

// let tree2 = BinaryNode {
//     value: 20,
//     right: Some(Rc::new(RefCell::new(BinaryNode {
//         value: 50,
//         right: None,
//         left: Some(Rc::new(RefCell::new(BinaryNode {
//             value: 30,
//             right: Some(Rc::new(RefCell::new(BinaryNode {
//                 value: 45,
//                 left: None,
//                 right: Some(Rc::new(RefCell::new(BinaryNode {
//                     value: 49,
//                     left: None,
//                     right: None,
//                 }))),
//             }))),
//             left: Some(Rc::new(RefCell::new(BinaryNode {
//                 value: 29,
//                 right: None,
//                 left: Some(Rc::new(RefCell::new(BinaryNode {
//                     value: 21,
//                     left: None,
//                     right: None,
//                 }))),
//             }))),
//         }))),
//     }))),
//     left: Some(Rc::new(RefCell::new(BinaryNode {
//         value: 10,
//         right: Some(Rc::new(RefCell::new(BinaryNode {
//             value: 15,
//             left: None,
//             right: None,
//         }))),
//         left: Some(Rc::new(RefCell::new(BinaryNode {
//             value: 5,
//             left: None,
//             right: Some(Rc::new(RefCell::new(BinaryNode {
//                 value: 7,
//                 left: None,
//                 right: None,
//             }))),
//         }))),
//     }))),
// };
