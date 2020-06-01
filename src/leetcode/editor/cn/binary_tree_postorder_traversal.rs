//! [145]二叉树的后序遍历
//给定一个二叉树，返回它的 后序 遍历。
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
//输出: [3,2,1]
//
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
// Related Topics 栈 树

use std::cell::RefCell;
use std::rc::Rc;

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
// Definition for a binary tree node.

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut stack = vec![node];
            let mut output = vec![];
            while let Some(node) = stack.pop() {
                output.push(node.borrow().val);
                if let Some(left) = node.borrow().left.clone() {
                    stack.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    stack.push(right);
                }
            }
            output.reverse();
            output
        } else {
            vec![]
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
