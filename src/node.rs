use std::boxed::Box;
use std::option::Option;

#[derive(Clone, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
}

impl Node {
    pub fn print(&self, level: usize) {
        println!("{}{}", " ".repeat(level), self.value);
        match &*self.left {
            Some(n) => {
                Node::print(&n, level + 1);
            }
            _ => {}
        }
        match &*self.right {
            Some(n) => {
                Node::print(&n, level + 1);
            }
            _ => {}
        }
    }
    pub fn recursive_revert(self) -> Node {
        let swapped_left = match *self.left {
            Some(n) => Some(Self::recursive_revert(n)),
            None => None,
        };
        let swapped_right = match *self.right {
            Some(n) => Some(Self::recursive_revert(n)),
            None => None,
        };
        Node {
            value: self.value,
            left: Box::new(swapped_right),
            right: Box::new(swapped_left),
        }
    }
}
