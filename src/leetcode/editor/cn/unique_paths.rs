//! [62]不同路径
//一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
//
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
//
// 问总共有多少条不同的路径？
//
//
//
// 例如，上图是一个7 x 3 的网格。有多少可能的路径？
//
//
//
// 示例 1:
//
// 输入: m = 3, n = 2
//输出: 3
//解释:
//从左上角开始，总共有 3 条路径可以到达右下角。
//1. 向右 -> 向右 -> 向下
//2. 向右 -> 向下 -> 向右
//3. 向下 -> 向右 -> 向右
//
//
// 示例 2:
//
// 输入: m = 7, n = 3
//输出: 28
//
//
//
// 提示：
//
//
// 1 <= m, n <= 100
// 题目数据保证答案小于等于 2 * 10 ^ 9
//
// Related Topics 数组 动态规划

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut d = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if i == 0 && j == 0 {
                    d[i][j] = 1;
                } else if i == 0 {
                    d[i][j] = d[i][j - 1];
                } else if j == 0 {
                    d[i][j] = d[i - 1][j];
                } else {
                    d[i][j] = d[i - 1][j] + d[i][j - 1];
                }
            }
        }
        d[n - 1][m - 1]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
