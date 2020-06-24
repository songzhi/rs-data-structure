//! [206]反转链表
//反转一个单链表。 
//
// 示例: 
//
// 输入: 1->2->3->4->5->NULL
//输出: 5->4->3->2->1->NULL 
//
// 进阶: 
//你可以迭代或递归地反转链表。你能否用两种方法解决这道题？ 
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
        ListNode {
            next: None,
            val,
        }
    }
}
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref()?.next.is_none() {
            return head;
        }
        let mut pre = head.unwrap();
        let mut curr = pre.as_mut().next.take().unwrap();
        while let Some(next) = curr.next.take() {
            curr.next = Some(pre);
            pre = curr;
            curr = next;
        }
        curr.next = Some(pre);
        Some(curr)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
