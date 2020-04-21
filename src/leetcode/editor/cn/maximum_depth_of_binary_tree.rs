//! [104]二叉树的最大深度
//给定一个二叉树，找出其最大深度。
//
// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
//
// 说明: 叶子节点是指没有子节点的节点。
//
// 示例：
//给定二叉树 [3,9,20,null,null,15,7]，
//
//     3
//   / \
//  9  20
//    /  \
//   15   7
//
// 返回它的最大深度 3 。
// Related Topics 树 深度优先搜索

//leetcode submit region begin(Prohibit modification and deletion)
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

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let left = root.borrow().left.clone();
                let right = root.borrow().right.clone();
                depth(left).max(depth(right)) + 1
            } else {
                0
            }
        }
        depth(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
