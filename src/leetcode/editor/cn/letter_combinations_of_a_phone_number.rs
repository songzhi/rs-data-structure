//! [17]电话号码的字母组合
//给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
//
// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
//
//
//
// 示例:
//
// 输入："23"
//输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
//
//
// 说明:
//尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
// Related Topics 字符串 回溯算法

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        static I2A_MAP: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        fn backtrack(first: usize, digits: &[usize], comb: &mut Vec<u8>, output: &mut Vec<String>) {
            if comb.len() == digits.len() {
                output.push(unsafe { String::from_utf8_unchecked(comb.to_vec()) })
            } else {
                for c in I2A_MAP[digits[first] - 2].bytes() {
                    comb.push(c);
                    backtrack(first + 1, digits, comb, output);
                    comb.pop();
                }
            }
        }
        let mut output = Vec::new();
        let digits = digits
            .bytes()
            .map(|c| (c - b'0') as usize)
            .collect::<Vec<usize>>();
        let mut comb = Vec::new();
        backtrack(0, &digits, &mut comb, &mut output);
        output
    }
}
//leetcode submit region end(Prohibit modification and deletion)
