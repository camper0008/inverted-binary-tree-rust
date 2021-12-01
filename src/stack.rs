

// todo

// Stack* newStack(size_t maxsize)
// void* stack_free(Stack* s)
// size_t stack_length(Stack* s)
// int stack_can_push(Stack* s)
// int stack_can_pop(Stack* s)
// void stack_push(Stack* s, void* v)
// void* stack_pop(Stack* s)
// char** stack_get_list(Stack* s)
//

use crate::node::NodeType;

struct Stack {
    m: [NodeType; 1024],
    i: i32,
}

impl Stack {
    pub const fn new() -> Self {
        Stack{
            i: 0,
            m: [NodeType::Null; 1024],
        }
    }
}
