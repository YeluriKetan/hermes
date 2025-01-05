use std::{cmp::Ordering, collections::BinaryHeap, rc::Rc};

struct Wrapped<T> {
    value: T,
    cmp: Rc<dyn Fn(&T, &T) -> Ordering>,
}

impl<T> PartialEq for Wrapped<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.cmp)(&self.value, &other.value) == Ordering::Equal
    }
}

impl<T> Eq for Wrapped<T> {}

impl<T> PartialOrd for Wrapped<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.cmp)(&self.value, &other.value))
    }
}

impl<T> Ord for Wrapped<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cmp)(&self.value, &other.value)
    }
}

pub struct PriorityQueue<T> {
    heap: BinaryHeap<Wrapped<T>>,
    cmp: Rc<dyn Fn(&T, &T) -> Ordering>,
}

impl<T> PriorityQueue<T> {
    pub fn new<F>(cmp: F) -> Self
    where
        F: Fn(&T, &T) -> Ordering + 'static,
    {
        PriorityQueue {
            heap: BinaryHeap::new(),
            cmp: Rc::new(cmp),
        }
    }

    pub fn push(&mut self, value: T) {
        let wrapped = Wrapped {
            value,
            cmp: self.cmp.clone(),
        };
        self.heap.push(wrapped);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|wrapped| wrapped.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|wrapped| &wrapped.value)
    }
}
