//! [43]字符串相乘
//给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
//
// 示例 1:
//
// 输入: num1 = "2", num2 = "3"
//输出: "6"
//
// 示例 2:
//
// 输入: num1 = "123", num2 = "456"
//输出: "56088"
//
// 说明：
//
//
// num1 和 num2 的长度小于110。
// num1 和 num2 只包含数字 0-9。
// num1 和 num2 均不以零开头，除非是数字 0 本身。
// 不能使用任何标准库的大数类型（比如 BigInteger）或直接将输入转换为整数来处理。
//
// Related Topics 数学 字符串



pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".into();
        }
        let mut ans = vec![0usize; num1.len() + num2.len()];
        for (i, c1) in num1.bytes().rev().enumerate() {
            for (j, c2) in num2.bytes().rev().enumerate() {
                ans[i + j] += (c1 - b'0') as usize * (c2 - b'0') as usize;
            }
        }
        for i in 0..ans.len() {
            if ans[i] > 9 {
                let n = ans[i];
                ans[i] = n % 10;
                ans[i + 1] += n / 10;
            }
        }
        ans.into_iter()
            .rev()
            .skip_while(|i| i.eq(&0))
            .map(|i| (i as u8 + b'0') as char)
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
