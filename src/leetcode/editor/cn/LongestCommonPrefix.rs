//! [14]最长公共前缀
//编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串 ""。
//
// 示例 1:
//
// 输入: ["flower","flow","flight"]
//输出: "fl"
//
//
// 示例 2:
//
// 输入: ["dog","racecar","car"]
//输出: ""
//解释: 输入不存在公共前缀。
//
//
// 说明:
//
// 所有输入只包含小写字母 a-z 。
// Related Topics 字符串

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs.into_iter();
        if let Some(first) = strs.next() {
            let mut prefix_end = first.len();
            for s in strs {
                if s.is_empty() {
                    return String::new();
                }
                for (i, (c1, c2)) in first.as_bytes().iter().zip(s.as_bytes()).enumerate() {
                    if i > prefix_end {
                        break;
                    } else if c1.ne(c2) {
                        prefix_end = std::cmp::min(prefix_end, i);
                        break;
                    } else if i == s.len() - 1 {
                        prefix_end = std::cmp::min(prefix_end, i + 1);
                        break;
                    }
                }
            }
            first.as_str()[..prefix_end].into()
        } else {
            String::new()
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
