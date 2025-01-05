struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    len: usize,
    capacity: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: None,
            len: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity must be greater than 0");
        }
        Stack {
            top: None,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.capacity != 0 && self.len == self.capacity {
            panic!("Stack is full");
        }
        let new_node = Box::new(Node {
            value: value,
            next: self.top.take(),
        });
        self.top = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
