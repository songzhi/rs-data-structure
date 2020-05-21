//! [208]实现 Trie (前缀树)
//实现一个 Trie (前缀树)，包含 insert, search, 和 startsWith 这三个操作。
//
// 示例:
//
// Trie trie = new Trie();
//
//trie.insert("apple");
//trie.search("apple");   // 返回 true
//trie.search("app");     // 返回 false
//trie.startsWith("app"); // 返回 true
//trie.insert("app");
//trie.search("app");     // 返回 true
//
// 说明:
//
//
// 你可以假设所有的输入都是由小写字母 a-z 构成的。
// 保证所有输入均为非空字符串。
//
// Related Topics 设计 字典树

use std::borrow::{Borrow, BorrowMut};
//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;

pub struct Solution;

struct Trie {
    root: RefCell<Box<Node>>,
}

struct Node {
    next: [Option<Box<Node>>; 26],
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            next: Default::default(),
            is_word: false,
        }
    }

    fn insert(root: &mut Box<Node>, word: &[u8]) {
        let mut node = root;
        for &c in word {
            if node.get_next_mut(c).is_none() {
                node.get_next_mut(c).replace(Box::new(Node::new()));
            }
            node = node.get_next_mut(c).as_mut().unwrap();
        }
        node.is_word = true;
    }
    fn search(root: &Box<Node>, word: &[u8]) -> bool {
        let mut node = root;
        for &c in word {
            if let Some(next) = node.get_next(c) {
                node = next;
            } else {
                return false;
            }
        }
        node.is_word
    }
    fn starts_with(root: &Box<Node>, prefix: &[u8]) -> bool {
        let mut node = root;
        for &c in prefix {
            if let Some(next) = node.get_next(c) {
                node = next;
            } else {
                return false;
            }
        }
        true
    }
    fn get_next(&self, c: u8) -> &Option<Box<Node>> {
        &self.next[(c - b'a') as usize]
    }
    fn get_next_mut(&mut self, c: u8) -> &mut Option<Box<Node>> {
        &mut self.next[(c - b'a') as usize]
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            root: RefCell::new(Box::new(Node::new())),
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&self, word: String) {
        Node::insert(self.root.borrow_mut().borrow_mut(), word.as_bytes())
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        Node::search(self.root.borrow().borrow(), word.as_bytes())
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        Node::starts_with(self.root.borrow().borrow(), prefix.as_bytes())
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
//leetcode submit region end(Prohibit modification and deletion)
