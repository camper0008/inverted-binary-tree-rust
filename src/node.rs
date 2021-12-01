#[derive(Copy, Clone, PartialEq)]
pub enum NodeType {
    Null,
    Node(Node),
}

#[derive(Copy, Clone, PartialEq)]
pub struct Node {
    pub value:  i32,
    pub left:   *mut NodeType,
    pub right:  *mut NodeType,
}

impl Node {
    pub fn new(value: i32, left: *mut NodeType, right: *mut NodeType) -> Self {
        Self {
            value,
            left,
            right,
        }
    }
    pub fn swap_children(&mut self) {
        let temp = self.left;
        self.left = self.right;
        self.right = temp;
    }
    pub fn print(self, level: usize) {
        println!("{}{}", " ".repeat(level), self.value);
        unsafe {
            match *self.left {
                NodeType::Node(_) => {
                    if let NodeType::Node(d) = *self.left {
                        Node::print(d, level + 1);                        
                    };
                },
                _ => {},
            }
        }
        unsafe {
            match *self.right {
                NodeType::Node(_) => {
                    if let NodeType::Node(d) = *self.right {
                        Node::print(d, level + 1);                        
                    };
                },
                _ => {},
            }
        }
    }
    /* 
    fn recursive_revert(&mut self) {

    } 
    fn stack_revert(&mut self) {

    }
    fn free_node(&mut self) {

    }
    */
}
