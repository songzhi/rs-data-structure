use crate::utils::width_in_fmt;
use core::cmp::max;
use core::fmt;
use core::fmt::Display;
use std::collections::vec_deque::VecDeque;
use std::marker::PhantomData;

pub type Link<T, Ty = BasicBinaryTreeType> = Option<Box<Node<T, Ty>>>;

pub trait BinaryTreeType {
    #[inline]
    fn is_searchable() -> bool {
        false
    }
    #[inline]
    fn is_avl() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BasicBinaryTreeType {}

impl BinaryTreeType for BasicBinaryTreeType {}

#[derive(Debug, Clone)]
pub struct Node<T, Ty = BasicBinaryTreeType>
where
    Ty: BinaryTreeType,
{
    pub elem: T,
    pub left: Link<T, Ty>,
    pub right: Link<T, Ty>,
    pub(crate) height: usize,
    _ty: PhantomData<Ty>,
}

#[derive(Debug, Clone, Default)]
pub struct BinaryTree<T, Ty = BasicBinaryTreeType>
where
    Ty: BinaryTreeType,
{
    pub(crate) root: Link<T, Ty>,
}

impl<T, Ty> BinaryTree<T, Ty>
where
    Ty: BinaryTreeType,
{
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn traverse_pre(&self, mut visit: impl FnMut(&T)) {
        if let Some(tree) = self.root.as_ref() {
            tree.traverse_pre(&mut visit);
        }
    }
    pub fn traverse_in(&self, mut visit: impl FnMut(&T)) {
        if let Some(tree) = self.root.as_ref() {
            tree.traverse_in(&mut visit);
        }
    }
    pub fn traverse_post(&self, mut visit: impl FnMut(&T)) {
        if let Some(tree) = self.root.as_ref() {
            tree.traverse_post(&mut visit);
        }
    }
    pub fn traverse_level(&self, mut visit: impl FnMut(&T)) {
        if let Some(tree) = self.root.as_ref() {
            tree.traverse_level(&mut visit);
        }
    }
    pub fn from_post_expr(
        tokens: impl Iterator<Item = T>,
        is_operator: impl Fn(&T) -> bool,
    ) -> Self {
        Self {
            root: Node::from_post_expr(tokens, is_operator),
        }
    }
    pub fn depth(&self) -> usize {
        self.root.as_ref().map(|root| root.depth()).unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn height(&self) -> usize {
        self.root.as_ref().map(|n| n.height()).unwrap_or(0)
    }
}

impl<T: PartialEq> BinaryTree<T, BasicBinaryTreeType> {
    pub fn from_seq_pre(mut seq_itr: impl Iterator<Item = T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_pre(&mut seq_itr, null_val),
        }
    }
    pub fn from_seq_in(mut seq_itr: impl Iterator<Item = T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_in(&mut seq_itr, null_val),
        }
    }
    pub fn from_seq_post(mut seq_itr: impl Iterator<Item = T>, null_val: &T) -> Self {
        Self {
            root: Node::from_seq_post(&mut seq_itr, null_val),
        }
    }
}

impl<T: Display, Ty> Display for BinaryTree<T, Ty>
where
    Ty: BinaryTreeType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref node) = self.root {
            node.fmt(f)
        } else {
            writeln!(f, "Empty Binary Tree")
        }
    }
}

#[inline]
fn unbox_link<T, Ty: BinaryTreeType>(link: &Link<T, Ty>) -> Option<&Node<T, Ty>> {
    link.as_ref().map(|node| &**node)
}

impl<T: Ord, Ty> Node<T, Ty>
where
    Ty: BinaryTreeType,
{
    pub fn find(&self, elem: &T) -> Option<&Node<T, Ty>> {
        if self.is_searchable() {
            if *elem < self.elem {
                unbox_link(&self.left)?.find(elem)
            } else if *elem > self.elem {
                unbox_link(&self.right)?.find(elem)
            } else {
                Some(self)
            }
        } else {
            let mut que = VecDeque::new();
            que.push_back(self);
            while let Some(node) = que.pop_front() {
                if node.elem == *elem {
                    return Some(node);
                }
                if let Some(node) = self.left.as_ref() {
                    que.push_back(&*node);
                }
                if let Some(node) = self.right.as_ref() {
                    que.push_back(&*node);
                }
            }
            None
        }
    }
    pub fn find_min(&self) -> &Node<T, Ty> {
        if self.is_searchable() {
            let mut node = self;
            while let Some(left) = unbox_link(&node.left) {
                node = left;
            }
            node
        } else {
            let mut min_node = self;
            let mut que = VecDeque::new();
            que.push_back(self);
            while let Some(node) = que.pop_front() {
                if node.elem < min_node.elem {
                    min_node = node;
                }
                if let Some(node) = self.left.as_ref() {
                    que.push_back(&*node);
                }
                if let Some(node) = self.right.as_ref() {
                    que.push_back(&*node);
                }
            }
            min_node
        }
    }
    pub fn find_max(&self) -> &Node<T, Ty> {
        if self.is_searchable() {
            let mut node = self;
            while let Some(right) = unbox_link(&node.right) {
                node = right;
            }
            node
        } else {
            let mut max_node = self;
            let mut que = VecDeque::new();
            que.push_back(self);
            while let Some(node) = que.pop_front() {
                if node.elem > max_node.elem {
                    max_node = node;
                }
                if let Some(node) = self.left.as_ref() {
                    que.push_back(&*node);
                }
                if let Some(node) = self.right.as_ref() {
                    que.push_back(&*node);
                }
            }
            max_node
        }
    }
}

impl<T, Ty> Node<T, Ty>
where
    Ty: BinaryTreeType,
{
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            height: 1,
            _ty: PhantomData,
        }
    }
    pub fn with_children(elem: T, left: Link<T, Ty>, right: Link<T, Ty>) -> Self {
        Self {
            elem,
            height: Self::calc_height(&left, &right),
            left,
            right,
            _ty: PhantomData,
        }
    }
    pub fn traverse_pre(&self, visit: &mut impl FnMut(&T)) {
        visit(&self.elem);
        if let Some(tree) = self.left.as_ref() {
            tree.traverse_pre(visit);
        }
        if let Some(tree) = self.right.as_ref() {
            tree.traverse_pre(visit);
        }
    }
    pub fn traverse_in(&self, visit: &mut impl FnMut(&T)) {
        if let Some(tree) = self.left.as_ref() {
            tree.traverse_in(visit);
        }
        visit(&self.elem);
        if let Some(tree) = self.right.as_ref() {
            tree.traverse_in(visit);
        }
    }
    pub fn traverse_post(&self, visit: &mut impl FnMut(&T)) {
        if let Some(tree) = self.left.as_ref() {
            tree.traverse_post(visit);
        }
        if let Some(tree) = self.right.as_ref() {
            tree.traverse_post(visit);
        }
        visit(&self.elem);
    }
    pub fn traverse_level(&self, visit: &mut impl FnMut(&T)) {
        let mut que = VecDeque::new();
        que.push_back(self);
        while let Some(node) = que.pop_front() {
            visit(&node.elem);
            if let Some(node) = self.left.as_ref() {
                que.push_back(&*node);
            }
            if let Some(node) = self.right.as_ref() {
                que.push_back(&*node);
            }
        }
    }
    pub fn from_post_expr(
        tokens: impl Iterator<Item = T>,
        is_operator: impl Fn(&T) -> bool,
    ) -> Link<T, Ty> {
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
    #[inline]
    pub fn depth(&self) -> usize {
        self.height
    }
    pub(crate) fn calc_height(left: &Link<T, Ty>, right: &Link<T, Ty>) -> usize {
        let left_height = left.as_ref().map(|m| m.height).unwrap_or(0);
        let right_height = right.as_ref().map(|m| m.height).unwrap_or(0);
        max(left_height, right_height) + 1
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
    #[inline]
    pub fn has_child(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }
    #[inline]
    pub fn is_searchable(&self) -> bool {
        Ty::is_searchable()
    }
    #[inline]
    pub fn is_avl(&self) -> bool {
        Ty::is_avl()
    }
}

impl<T: PartialEq, Ty> Node<T, Ty>
where
    Ty: BinaryTreeType,
{
    pub fn from_seq_pre(seq_itr: &mut impl Iterator<Item = T>, null_val: &T) -> Link<T, Ty> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            None
        } else {
            let mut tree = Box::new(Node::new(elem));
            tree.left = Self::from_seq_pre(seq_itr, null_val);
            tree.right = Self::from_seq_pre(seq_itr, null_val);
            tree.height = Self::calc_height(&tree.left, &tree.right);
            Some(tree)
        }
    }
    pub fn from_seq_in(seq_itr: &mut impl Iterator<Item = T>, null_val: &T) -> Link<T, Ty> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            None
        } else {
            let left = Self::from_seq_pre(seq_itr, null_val);
            let mut tree = Box::new(Node::new(elem));
            tree.left = left;
            tree.right = Self::from_seq_pre(seq_itr, null_val);
            tree.height = Self::calc_height(&tree.left, &tree.right);
            Some(tree)
        }
    }
    pub fn from_seq_post(seq_itr: &mut impl Iterator<Item = T>, null_val: &T) -> Link<T, Ty> {
        let elem = seq_itr.next()?;
        if elem == *null_val {
            None
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

impl<T: Display, Ty> Display for Node<T, Ty>
where
    Ty: BinaryTreeType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fill_map<'a, T, Ty>(
            map: &mut Vec<Option<&'a Node<T, Ty>>>,
            node: &'a Node<T, Ty>,
            index: usize,
        ) where
            Ty: BinaryTreeType,
        {
            map[index] = Some(node);
            if let Some(ref left) = node.left {
                fill_map(map, &*left, index * 2 + 1);
            }
            if let Some(ref right) = node.right {
                fill_map(map, &*right, (index + 1) * 2);
            }
        }

        fn print_left_to_parent_branch_top(f: &mut fmt::Formatter, w: usize) -> fmt::Result {
            write!(f, "{:w1$}{:_^w2$}/ ", " ", "_", w1 = w + 1, w2 = w - 3)
        }
        fn print_right_to_parent_branch_top(f: &mut fmt::Formatter, w: usize) -> fmt::Result {
            write!(f, "\\{:_^w1$}{:w2$}", "_", " ", w1 = w - 3, w2 = w + 2)
        }
        fn print_left_to_parent_branch_bottom(f: &mut fmt::Formatter, w: usize) -> fmt::Result {
            write!(f, "{:w1$}/{:w2$}", " ", " ", w1 = w, w2 = w - 1)
        }
        fn print_right_to_parent_branch_bottom(f: &mut fmt::Formatter, w: usize) -> fmt::Result {
            write!(f, "{:w1$}\\{:w2$}", " ", " ", w1 = w - 2, w2 = w + 1)
        }

        let depth = self.depth();
        let mut map = vec![None; 2usize.pow(depth as u32) - 1];
        fill_map(&mut map, self, 0);
        let mut index = 0usize;
        for j in 0..depth {
            let w = 2usize.pow((depth - j + 1) as u32);
            if j > 0 {
                // Top part of node to parent branch
                for i in 0..2usize.pow(j as u32) {
                    if map[index + i].is_some() {
                        if i % 2 == 0 {
                            print_left_to_parent_branch_top(f, w)?;
                        } else {
                            print_right_to_parent_branch_top(f, w)?;
                        }
                    } else {
                        write!(f, "{:w$}", "", w = w * 2)?;
                    }
                }
                writeln!(f)?;
                // Bottom part of node to parent branch
                for i in 0..2usize.pow(j as u32) {
                    if map[index + i].is_some() {
                        if i % 2 == 0 {
                            print_left_to_parent_branch_bottom(f, w)?;
                        } else {
                            print_right_to_parent_branch_bottom(f, w)?;
                        }
                    } else {
                        write!(f, "{:w$}", "", w = w * 2)?;
                    }
                }
                writeln!(f)?;
            }
            // Node content
            for _ in 0..2usize.pow(j as u32) {
                if let Some(node) = map[index] {
                    let content = format!("({})", node.elem);
                    write!(
                        f,
                        "{:^width$}",
                        content.as_str(),
                        width = width_in_fmt(content.as_str(), w * 2)
                    )?;
                } else {
                    write!(f, "{:w$}", "", w = w * 2)?;
                }
                index += 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::BinaryTree;

    #[test]
    fn from_seq_pre() {
        let seq = "ABC##DE#G##F###";
        let tree = BinaryTree::from_seq_pre(seq.chars(), &'#');
        let mut seq = String::new();
        tree.traverse_pre(|ch| seq.push(*ch));
        assert_eq!(seq, "ABCDEGF");
    }

    #[test]
    fn from_post_expr() {
        let tokens = "ab+cde+**";
        let is_operator = |token: &char| "+-*/".contains(*token);
        let tree = BinaryTree::<char>::from_post_expr(tokens.chars(), is_operator);

        let mut seq = String::new();
        tree.traverse_in(|ch| seq.push(*ch));
        assert_eq!(seq, "a+b*c*d+e");
    }

    #[test]
    fn depth() {
        let seq = "ABC##DE#G##F###";
        let tree = BinaryTree::from_seq_pre(seq.chars(), &'#');
        assert_eq!(5, tree.depth());
    }

    #[test]
    fn traverse_level() {
        let seq = "ABC##DE#G##F###";
        let tree = BinaryTree::from_seq_pre(seq.chars(), &'#');
        let mut seq = String::new();
        tree.traverse_level(|ch| seq.push(*ch));
        assert_eq!(seq, "ABCDEFG");
    }
}
