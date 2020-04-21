//! [78]子集
//给定一组不含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。 
//
// 说明：解集不能包含重复的子集。 
//
// 示例: 
//
// 输入: nums = [1,2,3]
//输出:
//[
//  [3],
//  [1],
//  [2],
//  [1,2,3],
//  [1,3],
//  [2,3],
//  [1,2],
//  []
//] 
// Related Topics 位运算 数组 回溯算法


pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], stack: &mut Vec<i32>, output: &mut Vec<Vec<i32>>, first: usize) {
            output.push(stack.to_vec());
            for i in first..nums.len() {
                stack.push(nums[i]);
                backtrack(nums, stack, output, i + 1);
                stack.pop();
            }
        }
        let mut stack = Vec::with_capacity(nums.len());
        let mut output = Vec::with_capacity(2usize.pow(nums.len() as u32));
        backtrack(&nums, &mut stack, &mut output, 0);
        output
    }
}
//leetcode submit region end(Prohibit modification and deletion)
