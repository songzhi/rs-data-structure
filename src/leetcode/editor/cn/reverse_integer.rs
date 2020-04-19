//! [7]整数反转
//给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//
// 示例 1:
//
// 输入: 123
//输出: 321
//
//
// 示例 2:
//
// 输入: -123
//输出: -321
//
//
// 示例 3:
//
// 输入: 120
//输出: 21
//
//
// 注意:
//
// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31, 2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
// Related Topics 数学

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            rev = rev
                .checked_mul(10)
                .and_then(|x| x.checked_add(pop))
                .unwrap_or(0);
        }
        rev
    }
}
//leetcode submit region end(Prohibit modification and deletion)
