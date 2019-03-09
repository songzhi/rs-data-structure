use std::cmp::max;
use std::collections::vec_deque::VecDeque;

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub left: Link<T>,
    pub right: Link<T>,
}

#[derive(Debug)]
pub struct BinTree<T> {
    root: Link<T>
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn traverse_pre(&self, mut visit: impl FnMut(&T)) {
        if let Some(ref tree) = self.root {
            tree.traverse_pre(&mut visit);
        }
    }
    pub fn traverse_in(&self, mut visit: impl FnMut(&T)) {
        if let Some(ref tree) = self.root {
            tree.traverse_in(&mut visit);
        }
    }
    pub fn traverse_post(&self, mut visit: impl FnMut(&T)) {
        if let Some(ref tree) = self.root {
            tree.traverse_post(&mut visit);
        }
    }
    pub fn traverse_level(&self, mut visit: impl FnMut(&T)) {
        if let Some(ref tree) = self.root {
            tree.traverse_level(&mut visit);
        }
    }
    pub fn from_post_expr(tokens: impl Iterator<Item=T>, is_operator: impl Fn(&T) -> bool) -> Self {
        Self {
            root: Node::from_post_expr(tokens, is_operator)
        }
    }
    pub fn depth(&self) -> usize {
        self.root.as_ref().map(|root| root.depth()).unwrap_or(0)
    }
}

impl<T: PartialEq> BinTree<T> {
    pub fn from_seq_pre(mut seq_itr: impl Iterator<Item=T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_pre(&mut seq_itr, null_val)
        }
    }
    pub fn from_seq_in(mut seq_itr: impl Iterator<Item=T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_in(&mut seq_itr, null_val)
        }
    }
    pub fn from_seq_post(mut seq_itr: impl Iterator<Item=T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_post(&mut seq_itr, null_val)
        }
    }
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
        }
    }
    pub fn with_children(elem: T, left: Link<T>, right: Link<T>) -> Self {
        Self {
            elem,
            left,
            right,
        }
    }
    pub fn traverse_pre(&self, visit: &mut impl FnMut(&T)) {
        visit(&self.elem);
        if let Some(ref node) = self.left {
            node.traverse_pre(visit);
        }
        if let Some(ref node) = self.right {
            node.traverse_pre(visit);
        }
    }
    pub fn traverse_in(&self, visit: &mut impl FnMut(&T)) {
        if let Some(ref node) = self.left {
            node.traverse_in(visit);
        }
        visit(&self.elem);
        if let Some(ref node) = self.right {
            node.traverse_in(visit);
        }
    }
    pub fn traverse_post(&self, visit: &mut impl FnMut(&T)) {
        if let Some(ref node) = self.left {
            node.traverse_post(visit);
        }
        if let Some(ref node) = self.right {
            node.traverse_post(visit);
        }
        visit(&self.elem);
    }
    pub fn traverse_level(&self, visit: &mut impl FnMut(&T)) {
        let mut que = VecDeque::new();
        que.push_back(self);
        while let Some(node) = que.pop_front() {
            visit(&node.elem);
            if let Some(ref left) = node.left {
                que.push_back(&*left);
            }
            if let Some(ref right) = node.right {
                que.push_back(&*right);
            }
        }
    }
    pub fn from_post_expr(tokens: impl Iterator<Item=T>, is_operator: impl Fn(&T) -> bool) -> Link<T> {
        let mut stack = vec![];
        for symbol in tokens {
            if is_operator(&symbol) {
                let right = stack.pop();
                let left = stack.pop();
                let new_tree = Box::new(Node::with_children(symbol, left, right));
                stack.push(new_tree);
            } else {
                let new_tree = Box::new(Node::new(symbol));
                stack.push(new_tree);
            }
        }
        stack.pop()
    }
    pub fn depth(&self) -> usize {
        let l_depth = self.left.as_ref().map(|node| node.depth()).unwrap_or(0);
        let r_depth = self.right.as_ref().map(|node| node.depth()).unwrap_or(0);
        max(l_depth, r_depth) + 1
    }
}

impl<T: PartialEq> Node<T> {
    pub fn from_seq_pre(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            return None;
        } else {
            let mut tree = Box::new(Node::new(elem));
            tree.left = Self::from_seq_pre(seq_itr, null_val);
            tree.right = Self::from_seq_pre(seq_itr, null_val);
            Some(tree)
        }
    }
    pub fn from_seq_in(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            return None;
        } else {
            let left = Self::from_seq_pre(seq_itr, null_val);
            let mut tree = Box::new(Node::new(elem));
            tree.left = left;
            tree.right = Self::from_seq_pre(seq_itr, null_val);
            Some(tree)
        }
    }
    pub fn from_seq_post(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            return None;
        } else {
            let left = Self::from_seq_pre(seq_itr, null_val);
            let right = Self::from_seq_pre(seq_itr, null_val);
            let mut tree = Box::new(Node::new(elem));
            tree.left = left;
            tree.right = right;
            Some(tree)
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        self.left.take();
        self.right.take();
    }
}

#[cfg(test)]
mod test {
    use super::BinTree;

    #[test]
    fn from_seq_pre() {
        let seq = "ABC##DE#G##F###";
        let tree = BinTree::from_seq_pre(seq.chars().into_iter(), &'#');
        let mut seq = String::new();
        tree.traverse_pre(|ch| seq.push(*ch));
        assert_eq!(seq, "ABCDEGF");
    }

    #[test]
    fn from_post_expr() {
        let tokens = "ab+cde+**";
        let is_operator = |token: &char| "+-*/".contains(*token);
        let tree = BinTree::from_post_expr(tokens.chars().into_iter(), is_operator);

        let mut seq = String::new();
        tree.traverse_in(|ch| seq.push(*ch));
        assert_eq!(seq, "a+b*c*d+e");
    }

    #[test]
    fn depth() {
        let seq = "ABC##DE#G##F###";
        let tree = BinTree::from_seq_pre(seq.chars().into_iter(), &'#');
        assert_eq!(5, tree.depth());
    }

    #[test]
    fn traverse_level() {
        let seq = "ABC##DE#G##F###";
        let tree = BinTree::from_seq_pre(seq.chars().into_iter(), &'#');
        let mut seq = String::new();
        tree.traverse_level(|ch| seq.push(*ch));
        assert_eq!(seq, "ABCDEFG");
    }
}