//!  用头尾链表存储表示法建立广义表，输出广义表，求广义表的表头、广义表的表尾和广 义表的深度。

use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub enum Node<T> {
    Atom(T),
    List(Link<T>, Link<T>), // List(head, tail)
}

impl<T> Node<T> {
    pub fn new_atom(data: T) -> Self {
        Node::Atom(data)
    }
    pub fn new_list(head: Link<T>, tail: Link<T>) -> Self {
        Node::List(head, tail)
    }
    pub fn is_atom(&self) -> bool {
        match self {
            Node::Atom(_) => true,
            _ => false
        }
    }
    pub fn is_empty(&self) -> bool {
        assert!(!self.is_atom());
        match self {
            Node::List(hp, _) => hp.is_none(),
            _ => false
        }
    }
    pub fn get_head(&self) -> Link<T> {
        match self {
            Node::Atom(_) => None,
            Node::List(hp, _) => hp.clone()
        }
    }
    pub fn get_tail(&self) -> Link<T> {
        match self {
            Node::Atom(_) => None,
            Node::List(_, tp) => tp.clone()
        }
    }
    pub fn depth(&self) -> usize {
        if self.is_atom() {
            return 0;
        }
        let mut max = 0;
        let mut node = self;
        while let Node::List(Some(hp), tp) = node {
            let dep = hp.as_ref().borrow().depth();
            if dep > max {
                max = dep;
            }
            if let Some(tp) = tp {
                node = unsafe { &*(tp.as_ref().as_ptr()) };
            } else {
                break;
            }
        }
        max + 1
    }
}