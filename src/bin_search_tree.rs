use crate::bintree::{Node, Link};
use std::fmt::Display;
use std::fmt;

pub struct BinSearchTree<T>
    where T: PartialEq + PartialOrd {
    pub root: Link<T>
}

impl<T> BinSearchTree<T>
    where T: PartialEq + PartialOrd {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }
    pub fn clear(&mut self) {
        self.root.take();
    }
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn find(&self, elem: T) -> Option<&Node<T>> {
        fn _find<T: PartialOrd>(node: Option<&Node<T>>, elem: T) -> Option<&Node<T>> {
            let node = node?;
            if elem < node.elem {
                _find(unbox_link(&node.left), elem)
            } else if elem > node.elem {
                _find(unbox_link(&node.right), elem)
            } else {
                Some(node)
            }
        }
        _find(unbox_link(&self.root), elem)
    }
    pub fn find_min(&self) -> Option<&Node<T>> {
        let mut node = unbox_link(&self.root)?;
        while let Some(left) = unbox_link(&node.left) {
            node = left;
        }
        Some(node)
    }
    pub fn find_max(&self) -> Option<&Node<T>> {
        let mut node = unbox_link(&self.root)?;
        while let Some(right) = unbox_link(&node.right) {
            node = right;
        }
        Some(node)
    }
    pub fn insert(&mut self, elem: T) {
        fn _insert<T: PartialOrd>(node: &mut Link<T>, elem: T) {
            if let Some(node) = node {
                if elem < node.elem {
                    _insert(&mut node.left, elem);
                } else if elem > node.elem {
                    _insert(&mut node.right, elem);
                } // Else elem is in the tree already; we'll do nothing
            } else {
                *node = Some(Box::new(Node::new(elem)));
            }
        }
        _insert(&mut self.root, elem);
    }
}

impl<T> Display for BinSearchTree<T>
    where T: Display + PartialEq + PartialOrd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref node) = self.root {
            node.fmt(f)
        } else {
            writeln!(f, "Empty Binary Search Tree")
        }
    }
}

fn unbox_link<T>(link: &Link<T>) -> Option<&Node<T>> {
    link.as_ref().map(|node| &**node)
}

fn unbox_link_mut<T>(link: &mut Link<T>) -> Option<&mut Node<T>> {
    link.as_mut().map(|node| &mut **node)
}

#[cfg(test)]
mod test {
    use crate::bin_search_tree::BinSearchTree;

    #[test]
    fn basics() {
        let tree: BinSearchTree<i32> = BinSearchTree::new();
        assert_eq!(true, tree.is_empty());
    }

    #[test]
    fn insert() {
        let mut tree = BinSearchTree::new();
        tree.insert(1);
        tree.insert(2);
        assert_eq!(false, tree.is_empty());
        assert_eq!(1, tree.root.as_ref().unwrap().elem);
        assert_eq!(2, tree.root.as_ref().unwrap().right.as_ref().unwrap().elem);
    }


}