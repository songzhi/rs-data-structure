//! [146]LRU缓存机制
//运用你所掌握的数据结构，设计和实现一个 LRU (最近最少使用) 缓存机制。它应该支持以下操作： 获取数据 get 和 写入数据 put 。
//
// 获取数据 get(key) - 如果密钥 (key) 存在于缓存中，则获取密钥的值（总是正数），否则返回 -1。
//写入数据 put(key, value) - 如果密钥已经存在，则变更其数据值；如果密钥不存在，则插入该组「密钥/数据值」。当缓存容量达到上限时，它应该在写
//入新数据之前删除最久未使用的数据值，从而为新的数据值留出空间。
//
//
//
// 进阶:
//
// 你是否可以在 O(1) 时间复杂度内完成这两种操作？
//
//
//
// 示例:
//
// LRUCache cache = new LRUCache( 2 /* 缓存容量 */ );
//
//cache.put(1, 1);
//cache.put(2, 2);
//cache.get(1);       // 返回  1
//cache.put(3, 3);    // 该操作会使得密钥 2 作废
//cache.get(2);       // 返回 -1 (未找到)
//cache.put(4, 4);    // 该操作会使得密钥 1 作废
//cache.get(1);       // 返回 -1 (未找到)
//cache.get(3);       // 返回  3
//cache.get(4);       // 返回  4
//
// Related Topics 设计

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type NonNullLink<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links,everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list,need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head) // +1 new_head
                // total: +2 new_head -- OK
            }
        }
        self.size += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // not emptying list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                    // total: -2 old
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn remove_node(&mut self, node: NonNullLink<T>) {
        let mut node = node.borrow_mut();
        match (node.prev.take(), node.next.take()) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next.replace(next.clone());
                next.borrow_mut().prev.replace(prev);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => unreachable!(),
        }
        self.size -= 1;
    }

    pub fn push_node_front(&mut self, node: NonNullLink<T>) {
        match self.head.take() {
            None => {
                let mut new_head = node.borrow_mut();
                new_head.next.take();
                new_head.prev.take();
                self.head.replace(node.clone());
                self.tail.replace(node.clone());
            }
            Some(old_head) => {
                old_head.borrow_mut().prev.replace(node.clone());
                let mut new_head = node.borrow_mut();
                new_head.prev.take();
                new_head.next.replace(old_head);
                self.head.replace(node.clone());
            }
        }
        self.size += 1;
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct LRUCache {
    list: RefCell<Deque<i32>>,
    hash: RefCell<HashMap<i32, NonNullLink<i32>>>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            list: RefCell::new(Deque::new()),
            hash: RefCell::new(HashMap::new()),
            capacity,
        }
    }
    pub fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.hash.borrow().get(&key) {
            let mut list = self.list.borrow_mut();
            list.remove_node(node.clone());
            list.push_node_front(node.clone());
            node.borrow().elem
        } else {
            -1
        }
    }

    pub fn put(&self, key: i32, value: i32) {
        let mut list = self.list.borrow_mut();
        let mut hash = self.hash.borrow_mut();
        if let Some(node) = hash.get(&key) {
            list.remove_node(node.clone());
            list.push_node_front(node.clone());
            node.borrow_mut().elem = value;
        } else {
            if list.size() == self.capacity as usize {
                if let Some(last) = list.pop_back() {
                    hash.remove(&last);
                }
            }
            let node = Node::new(key);
            hash.insert(key, node.clone());
            list.push_node_front(node);
        }
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
//leetcode submit region end(Prohibit modification and deletion)
