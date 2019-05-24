use super::bintree::{Node as CommonNode, Link as CommonLink, BinaryTreeType, BinaryTree};


#[derive(Copy, Clone, Debug)]
pub struct BinarySearchTreeType {}

impl BinaryTreeType for BinarySearchTreeType {
    fn is_searchable() -> bool { true }
}

type Link<T> = CommonLink<T, BinarySearchTreeType>;
type Node<T> = CommonNode<T, BinarySearchTreeType>;
pub type BinarySearchTree<T> = BinaryTree<T, BinarySearchTreeType>;


impl<T: Ord> BinarySearchTree<T> {
    pub fn clear(&mut self) -> Link<T> {
        self.root.take()
    }
    pub fn find(&self, elem: T) -> Option<&Node<T>> {
        unbox_link(&self.root)?.find(&elem)
    }
    pub fn find_min(&self) -> Option<&Node<T>> {
        Some(self.root.as_ref()?.find_min())
    }
    pub fn find_max(&self) -> Option<&Node<T>> {
        Some(self.root.as_ref()?.find_max())
    }
    pub fn insert(&mut self, elem: T) {
        if let Some(root) = self.root.as_mut() {
            root.insert(elem);
        } else {
            self.root = Some(Box::new(Node::new(elem)));
        }
    }
    pub fn delete(&mut self, elem: T) {
        let root = self.root.take();
        self.root = root.and_then(|n| n.delete(elem))
            .and_then(|n| Some(Box::new(n)));
    }
}

impl<T: Ord> Node<T> {

    fn insert(&mut self, elem: T) {
        if elem < self.elem {
            if let Some(left) = self.left.as_mut() {
                left.insert(elem);
            } else {
                self.left = Some(Box::new(Node::new(elem)));
            }
        } else if elem > self.elem {
            if let Some(right) = self.right.as_mut() {
                right.insert(elem);
            } else {
                self.right = Some(Box::new(Node::new(elem)));
            }
        } // Else elem is in the tree already; we'll do nothing
        self.height = Self::calc_height(&self.left, &self.right);
    }

    fn delete(mut self, elem: T) -> Option<Self> {
        if elem < self.elem {
            self.left = self.left.and_then(|n| n.delete(elem))
                .and_then(|n| Some(Box::new(n)));
        } else if elem > self.elem {
            self.right = self.right.and_then(|n| n.delete(elem))
                .and_then(|n| Some(Box::new(n)));
        } // Found element to be deleted
        else if self.left.is_some() && self.right.is_some() {
            // Two children
            let mut min_node_parent = unbox_link_mut(&mut self.right).unwrap(); // checked before
            while let Some(left) = unbox_link(&min_node_parent.left) {
                if !left.has_child() {
                    break;
                }
                min_node_parent = unbox_link_mut(&mut min_node_parent.left).unwrap(); // will never panic
            }
            let min_node = min_node_parent.left.take(); // take the min_node
            if let Some(mut node) = min_node {
                // exists and might has right subtree
                min_node_parent.left = node.right.take(); // replace itself by its right subtree
                self.elem = node.elem;
            } else {
                // the min_node_parent has not left subtree,so it's actually the min_node
                let min_node_parent = self.right.take().unwrap();
                self.elem = min_node_parent.elem;
                self.right = min_node_parent.right;
            }
        } else {
            // One or zero child
            let mut res: Option<Node<T>> = None;
            if self.left.is_none() {
                res = self.right.take().map(|mut n| {
                    n.height = Self::calc_height(&n.left, &n.right);
                    *n
                });
            }
            if self.right.is_none() {
                res = self.left.take().map(|mut n| {
                    n.height = Self::calc_height(&n.left, &n.right);
                    *n
                });
            }
            return res;
        }
        self.height = Self::calc_height(&self.left, &self.right);
        Some(self)
    }
}

#[inline]
fn unbox_link<T>(link: &Link<T>) -> Option<&Node<T>> {
    link.as_ref().map(|node| &**node)
}

#[inline]
fn unbox_link_mut<T>(link: &mut Link<T>) -> Option<&mut Node<T>> {
    link.as_mut().map(|node| &mut **node)
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn basics() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(true, tree.is_empty());
    }

    #[test]
    fn insert() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        assert_eq!(false, tree.is_empty());
        assert_eq!(1, tree.root.as_ref().unwrap().elem);
        assert_eq!(2, tree.root.as_ref().unwrap().right.as_ref().unwrap().elem);
    }

    #[test]
    fn height() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(1);
        tree.insert(2);
        tree.delete(1);
        tree.insert(8);
        tree.insert(10);

        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn find() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        assert_eq!(true, tree.find(1).is_some());
        assert_eq!(false, tree.find(3).is_some());
    }

    #[test]
    fn find_min() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(3);
        tree.insert(9);
        assert_eq!(2, tree.find_min().unwrap().elem);
    }

    #[test]
    fn find_max() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(3);
        tree.insert(9);
        assert_eq!(9, tree.find_max().unwrap().elem);
    }

    #[test]
    fn delete() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(3);
        tree.insert(9);

        assert_eq!(true, tree.find(2).is_some());
        tree.delete(2);
        assert_eq!(true, tree.find(2).is_none());

        tree.delete(3);
        tree.delete(5);
        tree.delete(9);

        assert_eq!(true, tree.is_empty());
    }
}