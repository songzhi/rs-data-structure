//! 约瑟夫（Josephus）环问题：编号为 1,2,3,…,n 的 n 个人按顺时针方向围坐一圈，每人持有一个密码（正整数）。
//! 一开始任选一个正整数作为报数的上限值 m，从第一个人开始按顺时针方向自 1 开始顺序报数,报到 m 时停止。
//! 报 m 的人出列，将他的密码作为新的 m 值，从他在顺时针方向上的下一人开始重新从 1 报数，如此下去，直到所有人全部 出列为止。
//! 建立 n 个人的单循环链表存储结构，运行结束后，输出依次出队的人的序号。

use std::rc::Rc;
use std::cell::RefCell;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
        }))
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elem.eq(&other.elem)
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}


impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                new_tail.borrow_mut().next = self.head.clone();
                old_tail.borrow_mut().next = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                new_tail.borrow_mut().next = Some(new_tail.clone());
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
}


pub fn josephus_ring(codes: &[usize], init_top: usize) -> Vec<usize> {
    let mut list = List::new();
    for (i, &code) in codes.iter().enumerate() {
        list.push_back((i, code));
    }
    let mut top = init_top;
    let mut rank = Vec::new();
    let mut node;
    if let Some(head) = list.head.clone() {
        node = head;
    } else {
        return rank;
    };
    while node != node.borrow().next.clone().unwrap() {
        for _ in 0..top - 2 {
            let tmp = node.borrow().next.clone().unwrap();
            node = tmp;
        }

        let next = node.borrow_mut().next.take().unwrap();
        node.borrow_mut().next = next.borrow().next.clone();

        rank.push(next.borrow().elem.0);
        top = next.borrow().elem.1;

        let tmp = node.borrow().next.clone().unwrap();
        node = tmp;
    }
    rank.push(node.borrow().elem.0);
    rank
}

#[cfg(test)]
mod test {
    use super::josephus_ring;

    #[test]
    fn test_josephus_ring() {
        let codes = [1, 2, 2];
        let rank = josephus_ring(&codes, 3);
        assert_eq!(vec![2, 1, 0], rank);
    }
}