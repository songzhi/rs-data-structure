use std::sync::atomic::Ordering::SeqCst;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
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
    pub fn from_post_expr(tokens: impl Iterator<Item=T>, is_operator: impl Fn(&T) -> bool) -> Self {
        Self {
            root: Node::from_post_expr(tokens, is_operator)
        }
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
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
        }
    }
    fn with_children(elem: T, left: Link<T>, right: Link<T>) -> Self {
        Self {
            elem,
            left,
            right,
        }
    }
    fn traverse_pre(&self, visit: &mut impl FnMut(&T)) {
        visit(&self.elem);
        if let Some(ref node) = self.left {
            node.traverse_pre(visit);
        }
        if let Some(ref node) = self.right {
            node.traverse_pre(visit);
        }
    }
    fn traverse_in(&self, visit: &mut impl FnMut(&T)) {
        if let Some(ref node) = self.left {
            node.traverse_in(visit);
        }
        visit(&self.elem);
        if let Some(ref node) = self.right {
            node.traverse_in(visit);
        }
    }
    fn traverse_post(&self, visit: &mut impl FnMut(&T)) {
        if let Some(ref node) = self.left {
            node.traverse_post(visit);
        }
        if let Some(ref node) = self.right {
            node.traverse_post(visit);
        }
        visit(&self.elem);
    }
    fn from_post_expr(tokens: impl Iterator<Item=T>, is_operator: impl Fn(&T) -> bool) -> Link<T> {
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
}

impl<T: PartialEq> Node<T> {
    fn from_seq_pre(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
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
    fn from_seq_in(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
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
    fn from_seq_post(seq_itr: &mut impl Iterator<Item=T>, null_val: &T) -> Link<T> {
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
}