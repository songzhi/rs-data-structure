//! [20]有效的括号
//给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
//
// 有效字符串需满足：
//
//
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
//
//
// 注意空字符串可被认为是有效字符串。
//
// 示例 1:
//
// 输入: "()"
//输出: true
//
//
// 示例 2:
//
// 输入: "()[]{}"
//输出: true
//
//
// 示例 3:
//
// 输入: "(]"
//输出: false
//
//
// 示例 4:
//
// 输入: "([)]"
//输出: false
//
//
// 示例 5:
//
// 输入: "{[]}"
//输出: true
// Related Topics 栈 字符串

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        } else if s.len() % 2 == 1 {
            return false;
        }
        let mut stack: Vec<u8> = vec![];
        for c in s.as_bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(*c),
                b')' | b']' | b'}' => {
                    let is_valid = stack
                        .pop()
                        .map(|prev| c.eq(&(prev + 1)) || c.eq(&(prev + 2)))
                        .unwrap_or(false);
                    if !is_valid {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
