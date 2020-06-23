//! [110]平衡二叉树
//给定一个二叉树，判断它是否是高度平衡的二叉树。
//
// 本题中，一棵高度平衡二叉树定义为：
//
//
// 一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过1。
//
//
// 示例 1:
//
// 给定二叉树 [3,9,20,null,null,15,7]
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
//
// 返回 true 。
//
//示例 2:
//
// 给定二叉树 [1,2,2,3,3,null,null,4,4]
//
//        1
//      / \
//     2   2
//    / \
//   3   3
//  / \
// 4   4
//
//
// 返回 false 。
//
//
// Related Topics 树 深度优先搜索

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
// leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        /// 返回值为-1表示不平衡, >0表示树的高度
        fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let left = max_depth(root.borrow().left.clone());
                let right = max_depth(root.borrow().right.clone());
                if left == -1 || right == -1 || (left - right).abs() > 1 {
                    -1
                } else {
                    left.max(right) + 1
                }
            } else {
                0
            }
        }
        max_depth(root) != -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
