//! [5]最长回文子串
//给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
//
// 示例 1：
//
// 输入: "babad"
//输出: "bab"
//注意: "aba" 也是一个有效答案。
//
//
// 示例 2：
//
// 输入: "cbbd"
//输出: "bb"
//
// Related Topics 字符串 动态规划

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let mut filled_s = vec![255u8];
        for c in s.bytes() {
            filled_s.push(c);
            filled_s.push(255);
        }
        let mut radius = vec![1; filled_s.len()]; // 回文半径数组
        let mut r = 0; // 最右回文右边界
        let mut c = 0; // 最右回文右边界的对称中心
        let mut max_len = 0;
        let mut max_c = 0;
        for i in 0..radius.len() {
            if r > i {
                radius[i] = radius[2 * c - i].min(r - i + 1)
            }
            while i + radius[i] < filled_s.len() && i >= radius[i] {
                if filled_s[i - radius[i]] == filled_s[i + radius[i]] {
                    radius[i] += 1;
                } else {
                    break;
                }
            }
            if i + radius[i] > r {
                r = i + radius[i] - 1;
                c = i;
            }
            if radius[i] > max_len {
                max_len = radius[i];
                max_c = c;
            }
        }
        filled_s[max_c + 1 - max_len..max_c + max_len]
            .iter()
            .copied()
            .filter(u8::is_ascii)
            .map(char::from)
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome("babad".into()), "aba");
    assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
}
