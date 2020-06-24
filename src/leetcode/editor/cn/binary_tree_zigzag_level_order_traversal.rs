//! [103]二叉树的锯齿形层次遍历
//给定一个二叉树，返回其节点值的锯齿形层次遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
//
// 例如：
//给定二叉树 [3,9,20,null,null,15,7],
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
//
//
// 返回锯齿形层次遍历如下：
//
// [
//  [3],
//  [20,9],
//  [15,7]
//]
//
// Related Topics 栈 树 广度优先搜索

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

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
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut curr_level_stack = vec![root.unwrap()];
        let mut next_level_stack = vec![];
        let mut toggle = false;
        while !curr_level_stack.is_empty() {
            for node in curr_level_stack.iter() {
                if let Some(left) = node.borrow().left.clone() {
                    next_level_stack.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    next_level_stack.push(right);
                }
            }
            res.push(if toggle {
                curr_level_stack
                    .drain(..)
                    .rev()
                    .map(|node| node.borrow().val)
                    .collect()
            } else {
                curr_level_stack
                    .drain(..)
                    .map(|node| node.borrow().val)
                    .collect()
            });
            curr_level_stack.extend(next_level_stack.drain(..));
            toggle = !toggle;
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
