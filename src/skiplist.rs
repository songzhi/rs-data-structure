use std::cell::RefCell;
use std::rc::Rc;

use rand;
use std::fmt::{Debug, Display};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

struct Node<K, V> {
    next: Vec<Link<K, V>>,
    score: K,
    data: Rc<RefCell<V>>,
}

impl<K, V> Node<K, V> {
    fn new(links: Vec<Link<K, V>>, score: K, data: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            next: links,
            score,
            data: Rc::new(RefCell::new(data)),
        }))
    }
}

pub struct SkipList<K, V> {
    head: Link<K, V>,
    tails: Vec<Link<K, V>>,
    max_level: usize,
    length: u64,
}

impl<K: PartialOrd + Copy, V> SkipList<K, V> {
    pub fn new_empty(max_level: usize) -> Self {
        Self {
            max_level,
            head: None,
            tails: vec![None; max_level + 1],
            length: 0,
        }
    }
    fn get_level(&self) -> usize {
        let mut n = 0;
        // bool = p(true) = 0.5
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
    pub fn append(&mut self, score: K, data: V) {
        let level = 1 + if self.head.is_none() {
            self.max_level // use the maximum level for the first node
        } else {
            self.get_level() // determine the level by coin flips
        };
        let new = Node::new(vec![None; level], score, data);
        // update the tails for each level
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        // this is the first node in the list
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    pub fn find(&self, score: K) -> Option<Rc<RefCell<V>>> {
        if let Some(ref head) = self.head {
            let mut start_level = self.max_level;
            let node = head.clone();
            let mut result = None;
            while node.borrow().next[start_level].is_some() {
                start_level -= 1;
            }
            let mut n = node;
            for level in (0..=start_level).rev() {
                loop {
                    let next = n.clone();
                    match next.borrow().next[level] {
                        Some(ref next) if next.borrow().score <= score => n = next.clone(),
                        _ => break,
                    };
                }
                if n.borrow().score == score {
                    let tmp = n.borrow();
                    result = Some(tmp.data.clone());
                    break;
                }
            }
            result
        } else {
            None
        }
    }
    pub fn iter_level(&self, level: usize) -> ListIterator<K, V> {
        ListIterator::new(self.head.clone(), level)
    }
}

impl<K: PartialOrd + Copy, V> IntoIterator for SkipList<K, V> {
    type Item = (K, Rc<RefCell<V>>);
    type IntoIter = ListIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}

pub struct ListIterator<K, V> {
    current: Link<K, V>,
    level: usize,
}

impl<K, V> ListIterator<K, V> {
    fn new(start_at: Link<K, V>, level: usize) -> Self {
        Self {
            current: start_at,
            level,
        }
    }
}

impl<K: PartialOrd + Copy, V> Iterator for ListIterator<K, V> {
    type Item = (K, Rc<RefCell<V>>);

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.score, current.data.clone()));
                current.next[self.level].clone()
            }
            _ => None,
        };
        result
    }
}

impl<K: Display + PartialOrd + Copy, V> std::fmt::Debug for SkipList<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.head {
            Some(ref _head) => {
                for level in (0..=self.max_level).rev() {
                    let _ = write!(f, "{}: ", level);
                    for n in self.iter_level(level) {
                        let _ = write!(f, "[{}] ", n.0);
                    }
                    let _ = writeln!(f);
                }
                Ok(())
            }
            None => write!(f, "The list is empty: []"),
        }
    }
}
