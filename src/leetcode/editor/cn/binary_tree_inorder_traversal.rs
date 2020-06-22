//! [94]二叉树的中序遍历
//给定一个二叉树，返回它的中序 遍历。 
//
// 示例: 
//
// 输入: [1,null,2,3]
//   1
//    \
//     2
//    /
//   3
//
//输出: [1,3,2] 
//
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？ 
// Related Topics 栈 树 哈希表

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut curr = root;
        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr.take() {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }
            let top = stack.pop().unwrap();
            res.push(top.borrow().val);
            curr = top.borrow().right.clone();
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
