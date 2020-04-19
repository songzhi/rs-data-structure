//! [15]三数之和
//! 双指针.先排序,然后从头开始迭代,每次迭代都从剩余序列两边开始双指针逼近.注意去重.
//给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？找出所有满足条件且不重复的三
//元组。
//
// 注意：答案中不可以包含重复的三元组。
//
//
//
// 示例：
//
// 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
//
//满足要求的三元组集合为：
//[
//  [-1, 0, 1],
//  [-1, -1, 2]
//]
//
// Related Topics 数组 双指针

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return vec![]; }
        use std::cmp::Ordering;
        nums.sort();
        let mut result = vec![];
        for (i, v) in nums.iter().enumerate() {
            if v.gt(&0) { break; }
            if i > 0 && nums[i] == nums[i - 1] { continue; } // 去重
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = v + nums[l] + nums[r];
                match sum.cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        // 去重
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)
