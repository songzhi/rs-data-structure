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

#[cfg(test)]
mod test {
    use crate::bin_search_tree::BinSearchTree;

    #[test]
    fn basics() {
        let tree: BinSearchTree<i32> = BinSearchTree::new();
        assert_eq!(true, tree.is_empty());
    }
}