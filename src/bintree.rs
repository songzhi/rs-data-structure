type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
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
}

impl<T: PartialEq> BinTree<T> {
    pub fn from_seq_pre(mut seq_itr: impl Iterator<Item=T>, null_val: &T) -> Self {
        let tree = Node::from_seq_pre(&mut seq_itr, null_val);
        Self {
            root: tree
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
}