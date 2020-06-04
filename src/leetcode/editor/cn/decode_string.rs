//! [394]字符串解码
//给定一个经过编码的字符串，返回它解码后的字符串。
//
// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
//
// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
//
// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
//
// 示例:
//
//
//s = "3[a]2[bc]", 返回 "aaabcbc".
//s = "3[a2[c]]", 返回 "accaccacc".
//s = "2[abc]3[cd]ef", 返回 "abcabccdcdcdef".
//
// Related Topics 栈 深度优先搜索

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut chars = s.chars().peekable();
        let mut stack = vec![];
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    // 获取一个数字并进栈
                    let mut digits = c.to_string();
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            chars.next();
                            digits.push(c);
                        } else {
                            break;
                        }
                    }
                    stack.push(digits);
                }
                'a'..='z' | 'A'..='Z' | '[' => {
                    // 获取一个字母并进栈
                    stack.push(c.to_string());
                }
                ']' => {
                    let mut sub = vec![];
                    while "[".ne(stack.last().unwrap()) {
                        sub.push(stack.pop().unwrap());
                    }
                    stack.pop();
                    sub.reverse();
                    let times: usize = stack.pop().unwrap().parse().unwrap();
                    stack.push(sub.join("").repeat(times));
                }
                _ => unreachable!(),
            }
        }
        stack.join("")
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn test() {
    let s = String::from("3[a2[c]]");
    Solution::decode_string(s);
}
