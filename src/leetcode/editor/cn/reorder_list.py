# [143]重排链表
# 给定一个单链表 L：L0→L1→…→Ln-1→Ln ， 
# 将其重新排列后变为： L0→Ln→L1→Ln-1→L2→Ln-2→… 
# 
#  你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。 
# 
#  示例 1: 
# 
#  给定链表 1->2->3->4, 重新排列为 1->4->2->3. 
# 
#  示例 2: 
# 
#  给定链表 1->2->3->4->5, 重新排列为 1->5->2->4->3. 
#  Related Topics 链表


class ListNode:
    def __init__(self, val=0, next: 'ListNode' = None):
        self.val = val
        self.next = next


# leetcode submit region begin(Prohibit modification and deletion)
from typing import Optional


def find_middle_node(head: Optional[ListNode]) -> Optional[ListNode]:
    """
    找出链表 Head 的中间结点,把链表从中间断成两个子链表
    :param head: 头节点
    :return: 中间节点
    """
    if head is None or head.next is None:
        return head
    fast = head  # 遍历链表的时候每次向前走两步
    slow = head  # 遍历链表的时候每次向前走一步
    slow_prev = head  # 当 fast 到链表尾时,slow 恰好指向链表的中间结点
    while fast is not None and fast.next is not None:
        slow_prev = slow
        slow = slow.next
        fast = fast.next.next
    slow_prev.next = None  # 断成两个独立的子链表
    return slow


def reverse(head: Optional[ListNode]) -> Optional[ListNode]:
    """
    翻转链表
    :param head:
    :return:
    """
    if head is None or head.next is None:
        return head
    prev = head  # 前驱结点
    curr = head.next  # 当前结点
    prev.next = None
    # 使当前遍历到的结点 curr 指向其前驱结点
    while curr is not None:
        next_ = curr.next
        curr.next = prev
        prev = curr
        curr = next_
    return prev


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """

        if head is None or head.next is None:
            return
        left = head
        right = reverse(find_middle_node(head))
        while left.next is not None:
            tmp = left.next
            left.next = right
            left = tmp
            tmp = right.next
            right.next = left
            right = tmp
        left.next = right

# leetcode submit region end(Prohibit modification and deletion)
