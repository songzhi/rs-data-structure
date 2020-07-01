//! [445]两数相加 II
//给你两个 非空 链表来代表两个非负整数。数字最高位位于链表开始位置。它们的每个节点只存储一位数字。将这两数相加会返回一个新的链表。
//
// 你可以假设除了数字 0 之外，这两个数字都不会以零开头。
//
//
//
// 进阶：
//
// 如果输入链表不能修改该如何处理？换句话说，你不能对列表中的节点进行翻转。
//
//
//
// 示例：
//
// 输入：(7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
//输出：7 -> 8 -> 0 -> 7
//
// Related Topics 链表

pub struct Solution;
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
// Definition for singly-linked list.

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut s1 = vec![];
        let mut s2 = vec![];
        while let Some(node) = l1 {
            s1.push(node.val);
            l1 = node.next;
        }
        while let Some(node) = l2 {
            s2.push(node.val);
            l2 = node.next;
        }
        let mut head = None;
        let mut carry = 0;
        while !s1.is_empty() || !s2.is_empty() || carry != 0 {
            let n = s1.pop().unwrap_or(0) + s2.pop().unwrap_or(0) + carry;
            carry = n / 10;
            head = Some(Box::new(ListNode {
                val: n % 10,
                next: head,
            }));
        }
        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)
