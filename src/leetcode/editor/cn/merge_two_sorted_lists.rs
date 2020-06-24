//! [21]合并两个有序链表
//将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//
// 示例：
//
// 输入：1->2->4, 1->3->4
//输出：1->1->2->3->4->4
//
// Related Topics 链表

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

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev_head = ListNode::new(-1);
        let mut prev = &mut prev_head;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let tmp = l1.as_mut().unwrap().next.take();
                prev.next = l1;
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                prev.next = l2;
                l2 = tmp;
            }
            prev = prev.next.as_mut().unwrap();
        }
        prev.next = if l1.is_some() { l1 } else { l2 };
        prev_head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
