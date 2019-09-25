use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            next: AtomicPtr::default(),
        }
    }
}

type Link<T> = AtomicPtr<Node<T>>;

#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
    count: AtomicUsize,
}

unsafe impl<T> Send for List<T> {}

impl<T> List<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.count.load(Ordering::Acquire)
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn push(&mut self, elem: T) {
        let head = self.head.load(Ordering::Relaxed);
        let mut node = Node::new(elem);
        loop {
            node.next.store(head, Ordering::Relaxed);
            if self.head.compare_exchange(head, &mut node, Ordering::Release, Ordering::Relaxed).is_ok() {
                self.count.fetch_add(1, Ordering::AcqRel);
                break;
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let head = self.head.load(Ordering::Relaxed);
            while !head.is_null() && self.head.compare_exchange(head, (*head).next.load(Ordering::Relaxed), Ordering::Release, Ordering::Relaxed).is_err() {
                continue;
            }
            if head.is_null() {
                None
            } else {
                Some(head.read().elem)
            }
        }
    }
}