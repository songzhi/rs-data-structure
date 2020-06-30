//! [83]删除排序链表中的重复元素
//给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
//
// 示例 1:
//
// 输入: 1->1->2
//输出: 1->2
//
//
// 示例 2:
//
// 输入: 1->1->2->3->3
//输出: 1->2->3
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut prev_val: i32 = i32::min_value();
        let mut curr_node = &mut new_head;
        while let Some(node) = head.take() {
            if prev_val != node.val {
                if let Some(ref mut n) = curr_node {
                    n.next = Some(node.clone());
                    curr_node = &mut n.next;
                    prev_val = node.val;
                }
            }
            head = node.next;
        }
        if let Some(ref mut n) = curr_node {
            n.next = None;
        }
        new_head.unwrap().next
    }
}
//leetcode submit region end(Prohibit modification and deletion)


