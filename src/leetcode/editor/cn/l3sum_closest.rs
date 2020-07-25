//! [16]最接近的三数之和

//给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。返回这三个数的和
//。假定每组输入只存在唯一答案。
//
//
//
// 示例：
//
// 输入：nums = [-1,2,1,-4], target = 1
//输出：2
//解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
//
//
//
//
// 提示：
//
//
// 3 <= nums.length <= 10^3
// -10^3 <= nums[i] <= 10^3
// -10^4 <= target <= 10^4
//
// Related Topics 数组 双指针
// 👍 516 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        nums.sort();
        let mut closest_sum = 10i32.pow(7);
        for (i, &v) in nums.iter().enumerate() {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = v + nums[l] + nums[r];
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        return sum;
                    }
                }
            }
        }
        closest_sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
