//! [10]正则表达式匹配
//给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
//
// '.' 匹配任意单个字符
//'*' 匹配零个或多个前面的那一个元素
//
//
// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
//
// 说明:
//
//
// s 可能为空，且只包含从 a-z 的小写字母。
// p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
//
//
// 示例 1:
//
// 输入:
//s = "aa"
//p = "a"
//输出: false
//解释: "a" 无法匹配 "aa" 整个字符串。
//
//
// 示例 2:
//
// 输入:
//s = "aa"
//p = "a*"
//输出: true
//解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
//
//
// 示例 3:
//
// 输入:
//s = "ab"
//p = ".*"
//输出: true
//解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
//
//
// 示例 4:
//
// 输入:
//s = "aab"
//p = "c*a*b"
//输出: true
//解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
//
//
// 示例 5:
//
// 输入:
//s = "mississippi"
//p = "mis*is*p*."
//输出: false
// Related Topics 字符串 动态规划 回溯算法

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        use std::collections::HashMap;
        fn dp(
            memo: &mut HashMap<(usize, usize), bool>,
            i: usize,
            j: usize,
            text: &[u8],
            pattern: &[u8],
        ) -> bool {
            if let Some(&matched) = memo.get(&(i, j)) {
                return matched;
            }
            if j == pattern.len() {
                return i == text.len();
            }
            let first_matched = i < text.len() && (pattern[j] == text[i] || pattern[j] == b'.');
            let matched = if j + 1 < pattern.len() && pattern[j + 1] == b'*' {
                dp(memo, i, j + 2, text, pattern)
                    || first_matched && dp(memo, i + 1, j, text, pattern)
            } else {
                first_matched && dp(memo, i + 1, j + 1, text, pattern)
            };
            memo.insert((i, j), matched);
            matched
        }
        let mut memo = HashMap::new();
        dp(&mut memo, 0, 0, s.as_bytes(), p.as_bytes())
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn test() {
    let s = String::from("aa");
    let p = String::from("a");
    Solution::is_match(s, p);
}
