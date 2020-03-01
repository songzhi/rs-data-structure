use bitvec::prelude::BitVec;
use std::rc::Rc;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    symbols: Vec<Rc<T>>,
    weight: usize,
}

impl<T: Eq> Node<T> {
    fn new_leaf(symbol: Rc<T>, weight: usize) -> Self {
        Self {
            left: None,
            right: None,
            symbols: vec![symbol],
            weight,
        }
    }
    fn with_children(left: Box<Node<T>>, right: Box<Node<T>>) -> Self {
        Self {
            weight: left.weight + right.weight,
            symbols: [&left.symbols[..], &right.symbols[..]].concat(),
            left: Some(left),
            right: Some(right),
        }
    }
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub struct HuffmanTree<T> {
    root: Box<Node<T>>,
}

impl<'a, T: Eq + 'a> HuffmanTree<T> {
    pub fn new(leaves: Vec<(T, usize)>) -> Self {
        let mut nodes: Vec<Box<Node<T>>> = leaves
            .into_iter()
            .map(|(symbol, weight)| Box::new(Node::new_leaf(Rc::new(symbol), weight)))
            .collect();
        while nodes.len() != 1 {
            nodes.sort_by(|a, b| b.weight.cmp(&a.weight));
            let left = nodes.pop().unwrap();
            let right = nodes.pop().unwrap();
            nodes.push(Box::new(Node::with_children(left, right)));
        }
        Self {
            root: nodes.pop().unwrap(),
        }
    }
    pub fn encode(&self, symbols: impl Iterator<Item = &'a T>) -> BitVec {
        symbols.map(|s| self.encode_symbol(s)).fold(
            BitVec::with_capacity(self.root.symbols.len() * 8),
            |mut prev, mut current| {
                prev.append(&mut current);
                prev
            },
        )
    }
    pub fn encode_symbol(&self, symbol: &T) -> BitVec {
        let mut result: BitVec = BitVec::new();
        let mut tree = self.root.as_ref();
        while !tree.is_leaf() {
            if tree
                .left
                .as_ref()
                .unwrap()
                .symbols
                .iter()
                .filter(|&s| (**s).eq(symbol))
                .count()
                != 0
            {
                result.push(false);
                tree = tree.left.as_ref().unwrap();
            } else {
                result.push(true);
                tree = &tree.right.as_ref().unwrap();
            };
        }
        result
    }
    pub fn decode(&self, bits: BitVec) -> Vec<Rc<T>> {
        let mut symbols = Vec::new();
        let mut tree = self.root.as_ref();
        for bit in bits {
            tree = if bit {
                tree.right.as_ref().unwrap()
            } else {
                tree.left.as_ref().unwrap()
            };
            if tree.is_leaf() {
                symbols.extend(tree.symbols.clone().into_iter());
                tree = self.root.as_ref();
            }
        }
        symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let leaves = vec![
            ('A', 8),
            ('B', 3),
            ('C', 1),
            ('D', 1),
            ('E', 1),
            ('F', 1),
            ('G', 1),
            ('H', 1),
        ];
        let tree = HuffmanTree::new(leaves);
        let chars: Vec<char> = "ACDF".chars().collect();
        let bits = tree.encode(chars.iter());
        let chars: String = tree.decode(bits).into_iter().map(|c| *c).collect();

        assert_eq!("ACDF", chars.as_str());
    }
}
