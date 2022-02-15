mod node;

use crate::node::Node;

fn main() {
    let mut tree = Node {
        value: 9,
        right: Box::new(Some(Node {
            value: 2,
            left: Box::new(Some(Node {
                value: 6,
                left: Box::new(None),
                right: Box::new(None),
            })),
            right: Box::new(Some(Node {
                value: 12,
                left: Box::new(Some(Node {
                    value: 2,
                    left: Box::new(None),
                    right: Box::new(None),
                })),
                right: Box::new(Some(Node {
                    value: 4,
                    left: Box::new(None),
                    right: Box::new(None),
                })),
            })),
        })),
        left: Box::new(Some(Node {
            value: 2,
            left: Box::new(None),
            right: Box::new(None),
        })),
    };

    tree.print(0);
    tree = tree.recursive_revert();
    tree.print(0);
}
