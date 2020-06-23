//! [102]二叉树的层序遍历
//给你一个二叉树，请你返回其按 层序遍历 得到的节点值。 （即逐层地，从左到右访问所有节点）。
//
//
//
// 示例：
//二叉树：[3,9,20,null,null,15,7],
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
//
//
// 返回其层次遍历结果：
//
// [
//  [3],
//  [9,20],
//  [15,7]
//]
//
// Related Topics 树 广度优先搜索

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut curr_level_stack = vec![root.unwrap()];
        let mut next_level_stack = vec![];
        while !curr_level_stack.is_empty() {
            for node in curr_level_stack.iter() {
                if let Some(left) = node.borrow().left.clone() {
                    next_level_stack.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    next_level_stack.push(right);
                }
            }
            res.push(
                curr_level_stack
                    .drain(..)
                    .map(|node| node.borrow().val)
                    .collect(),
            );
            curr_level_stack.extend(next_level_stack.drain(..));
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
