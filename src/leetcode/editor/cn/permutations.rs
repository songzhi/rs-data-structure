//! [46]全排列
//给定一个 没有重复 数字的序列，返回其所有可能的全排列。
//
// 示例:
//
// 输入: [1,2,3]
//输出:
//[
//  [1,2,3],
//  [1,3,2],
//  [2,1,3],
//  [2,3,1],
//  [3,1,2],
//  [3,2,1]
//]
// Related Topics 回溯算法

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &mut Vec<i32>, first: usize, output: &mut Vec<Vec<i32>>, len: usize) {
            if first == len {
                output.push(nums.to_vec());
            }
            for i in first..len {
                nums.swap(i, first);
                backtrack(nums, first + 1, output, len);
                nums.swap(i, first);
            }
        }
        let len = nums.len();
        let mut output = vec![];
        backtrack(&mut nums, 0, &mut output, len);
        output
    }
}
//leetcode submit region end(Prohibit modification and deletion)
