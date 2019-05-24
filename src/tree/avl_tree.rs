use super::binary_tree::{Node as CommonNode, Link as CommonLink, BinaryTreeType, BinaryTree};

#[derive(Copy, Clone, Debug)]
pub struct AVLTreeType {}

impl BinaryTreeType for AVLTreeType {
    #[inline]
    fn is_searchable() -> bool { true }
    #[inline]
    fn is_avl() -> bool { true }
}

type Link<T> = CommonLink<T, AVLTreeType>;
type Node<T> = CommonNode<T, AVLTreeType>;

pub type AVLTree<T> = BinaryTree<T, AVLTreeType>;

impl<T: Ord> AVLTree<T> {
    pub fn insert(&mut self, elem: T) {
        if let Some(root) = self.root.take() {
            self.root = root.insert(elem);
        } else {
            self.root = Some(Box::new(Node::new(elem)));
        }
    }
}

impl<T: Ord> Node<T> {
    fn insert(mut self, elem: T) -> Link<T> {
        if elem < self.elem {
            if let Some(left) = self.left.take() {
                self.left = left.insert(elem);
            } else {
                self.left = Some(Box::new(Node::new(elem)));
            }
        } else if elem > self.elem {
            if let Some(right) = self.right.take() {
                self.right = right.insert(elem);
            } else {
                self.right = Some(Box::new(Node::new(elem)));
            }
        } // Else elem is in the tree already; we'll do nothing
        self.height = Self::calc_height(&self.left, &self.right);
        self.rebalance()
    }
    fn rotate_left(mut self) -> Link<T> {
        let mut right = self.right?;
        self.right = right.left;
        self.height = Self::calc_height(&self.left, &self.right);
        right.left = Some(Box::new(self));
        right.height = Self::calc_height(&right.left, &right.right);
        Some(right)
    }
    fn rotate_right(mut self) -> Link<T> {
        let mut left = self.left?;
        self.left = left.right;
        self.height = Self::calc_height(&self.left, &self.right);
        left.right = Some(Box::new(self));
        left.height = Self::calc_height(&left.left, &left.right);
        Some(left)
    }
    fn balance_factor(&self) -> isize {
        let left_height = self.left.as_ref().map(|n| n.height() as isize).unwrap_or(0);
        let right_height = self.right.as_ref().map(|n| n.height() as isize).unwrap_or(0);
        left_height - right_height
    }
    fn rebalance(mut self) -> Link<T> {
        let factor = self.balance_factor();
        if factor > 1 && self.left.as_ref()?.balance_factor() > 0 {
            self.rotate_right() // LL
        } else if factor > 1 && self.left.as_ref()?.balance_factor() <= 0 {
            self.left = self.left?.rotate_left();
            self.rotate_right() // LR
        } else if factor < -1 && self.right.as_ref()?.balance_factor() <= 0 {
            self.rotate_left() // RR
        } else if factor < -1 && self.right.as_ref()?.balance_factor() > 0 {
            self.right = self.right?.rotate_right();
            self.rotate_left() // RL
        } else {
            Some(Box::new(self))
        }
    }
}