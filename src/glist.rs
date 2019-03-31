use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub enum Node<T> {
    Atom(Rc<RefCell<T>>),
    List(Link<T>, Link<T>), // List(head, tail)
}

impl<T> Node<T> {
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
            Node::List(hp, _) => hp.map(|p| p.clone())
        }
    }
    pub fn get_tail(&self) -> Link<T> {
        match self {
            Node::Atom(_) => None,
            Node::List(_, tp) => tp.map(|p| p.clone())
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
                node = tp.as_ref().get_mut();
            } else {
                break;
            }
        }
        max + 1
    }
}