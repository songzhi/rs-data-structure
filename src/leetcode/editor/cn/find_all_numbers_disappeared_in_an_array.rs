//! [448]找到所有数组中消失的数字

pub struct Solution;

//给定一个范围在 1 ≤ a[i] ≤ n ( n = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。
//
// 找到所有在 [1, n] 范围之间没有出现在数组中的数字。
//
// 您能在不使用额外空间且时间复杂度为O(n)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。
//
// 示例:
//
//
//输入:
//[4,3,2,7,8,2,3,1]
//
//输出:
//[5,6]
//
// Related Topics 数组
// 👍 404 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let next_index = (nums[i].abs() - 1) as usize;
            nums[next_index] = -nums[next_index].abs();
        }
        nums.into_iter()
            .enumerate()
            .filter(|(_, n)| n.is_positive())
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
