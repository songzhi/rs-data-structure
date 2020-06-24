//! [22]括号生成
//数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
//
//
//
// 示例：
//
// 输入：n = 3
//输出：[
//       "((()))",
//       "(()())",
//       "(())()",
//       "()(())",
//       "()()()"
//     ]
//
// Related Topics 字符串 回溯算法

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(
            line: &mut Vec<u8>,
            output: &mut Vec<String>,
            open: usize,
            close: usize,
            n: usize,
        ) {
            if line.len() == n * 2 {
                output.push(unsafe { String::from_utf8_unchecked(line.to_vec()) });
                return;
            }
            if open < n {
                line.push(b'(');
                backtrack(line, output, open + 1, close, n);
                line.pop();
            }
            if close < open {
                line.push(b')');
                backtrack(line, output, open, close + 1, n);
                line.pop();
            }
        }
        let mut output = Vec::new();
        let mut line = Vec::new();
        backtrack(&mut line, &mut output, 0, 0, n as usize);
        output
    }
}
//leetcode submit region end(Prohibit modification and deletion)
