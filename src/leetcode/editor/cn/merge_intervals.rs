//! [56]合并区间

//给出一个区间的集合，请合并所有重叠的区间。
//
// 示例 1:
//
// 输入: [[1,3],[2,6],[8,10],[15,18]]
//输出: [[1,6],[8,10],[15,18]]
//解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
//
//
// 示例 2:
//
// 输入: [[1,4],[4,5]]
//输出: [[1,5]]
//解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。
// Related Topics 排序 数组
// 👍 515 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|r| r[0]);
        let mut merged: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if last[1] < interval[0] {
                    merged.push(interval);
                } else {
                    last[1] = last[1].max(interval[1]);
                }
            } else {
                merged.push(interval);
            }
        }
        merged
    }
}
//leetcode submit region end(Prohibit modification and deletion)
