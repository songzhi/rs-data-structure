//! [124]二叉树中的最大路径和
//给定一个非空二叉树，返回其最大路径和。
//
// 本题中，路径被定义为一条从树中任意节点出发，达到任意节点的序列。该路径至少包含一个节点，且不一定经过根节点。
//
// 示例 1:
//
// 输入: [1,2,3]
//
//       1
//      / \
//     2   3
//
//输出: 6
//
//
// 示例 2:
//
// 输入: [-10,9,20,null,null,15,7]
//
//   -10
//   / \
//  9  20
//    /  \
//   15   7
//
//输出: 42
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
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        /// 返回（单边最大值(一定和子节点连通)，单边或者两个单边+根的值（子树最大值，不一定和子节点连通））
        fn _max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(root) = root {
                let (left_single_max, left_max) = _max_path_sum(root.borrow().left.clone());
                let (right_single_max, right_max) = _max_path_sum(root.borrow().right.clone());
                (
                    (left_single_max.max(right_single_max) + root.borrow().val).max(0),
                    left_max
                        .max(right_max)
                        .max(left_single_max + right_single_max + root.borrow().val),
                )
            } else {
                (0, std::i32::MIN)
            }
        }
        _max_path_sum(root).1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
