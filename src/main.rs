mod node;
mod stack;

use crate::node::NodeType;
use crate::node::Node;

fn main() {

    let l = Node::new(1, &mut NodeType::Null, &mut NodeType::Null);
    let r = Node::new(2, &mut NodeType::Null, &mut NodeType::Null);
    let node = &mut Node::new(5, &mut NodeType::Node(r), &mut NodeType::Node(l));
    Node::print(*node, 0);
    Node::swap_children(node);
    Node::print(*node, 0);
}
