//! [2]两数相加
//给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
//
// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
//
// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//
// 示例：
//
// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//输出：7 -> 0 -> 8
//原因：342 + 465 = 807
//
// Related Topics 链表 数学

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut has_carry = false;
        let mut add_two_number =
            |l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>| match (l1, l2) {
                (Some(l1), Some(l2)) => {
                    let val = l1.val + l2.val + has_carry as i32;
                    has_carry = val >= 10;
                    (l1.next, l2.next, Some(Box::new(ListNode::new(val % 10))))
                }
                (Some(l1), None) => {
                    let val = l1.val + has_carry as i32;
                    has_carry = val >= 10;
                    (l1.next, None, Some(Box::new(ListNode::new(val % 10))))
                }
                (None, Some(l2)) => {
                    let val = l2.val + has_carry as i32;
                    has_carry = val >= 10;
                    (None, l2.next, Some(Box::new(ListNode::new(val % 10))))
                }
                (None, None) => {
                    let n = if has_carry {
                        Some(Box::new(ListNode::new(1)))
                    } else {
                        None
                    };
                    has_carry = false;
                    (None, None, n)
                }
            };
        if let (mut l1, mut l2, Some(mut head)) = add_two_number(l1, l2) {
            let mut tail = &mut head;
            loop {
                let (l1_next, l2_next, current) = add_two_number(l1, l2);
                l1 = l1_next;
                l2 = l2_next;
                if let Some(current) = current {
                    tail = tail.next.get_or_insert(current);
                } else {
                    break;
                }
            }
            Some(head)
        } else {
            None
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
