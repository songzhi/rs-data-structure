//! [11]盛最多水的容器
//给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i,
//ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
//
// 说明：你不能倾斜容器，且 n 的值至少为 2。
//
//
//
//
//
// 图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
//
//
//
// 示例：
//
// 输入：[1,8,6,2,5,4,8,3,7]
//输出：49
// Related Topics 数组 双指针

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = heights.len() - 1;
        let mut max_area = 0;
        while i != j {
            max_area = max_area.max(heights[i].min(heights[j]) * (j - i) as i32);
            if heights[i] < heights[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_area
    }
}
//leetcode submit region end(Prohibit modification and deletion)
