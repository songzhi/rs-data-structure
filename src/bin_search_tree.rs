use crate::bintree::{Node, Link};
use std::fmt::Display;
use std::fmt;

pub struct BinSearchTree<T>
    where T: PartialEq + PartialOrd {
    root: Link<T>
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

#[cfg(test)]
mod test {
    use crate::bin_search_tree::BinSearchTree;

    #[test]
    fn basics() {
        let tree: BinSearchTree<i32> = BinSearchTree::new();
        assert_eq!(true, tree.is_empty());
    }
}