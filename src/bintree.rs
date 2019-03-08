type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct BinTree<T> {
    tree: Link<T>
}

impl<T> BinTree<T> {
    fn new() -> Self {
        Self {
            tree: None
        }
    }
    fn traverse_pre<F: Fn(&T)>(&self, visit: F) {
        if let Some(ref tree) = self.tree {
            tree.traverse_pre(visit);
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
    fn traverse_pre<F: Fn(&T)>(&self, visit: F) {
        visit(&self.elem);
        if let Some(ref node) = self.left {
            node.traverse_pre(&visit);
        }
        if let Some(ref node) = self.right {
            node.traverse_pre(&visit);
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        self.left.take();
        self.right.take();
    }
}