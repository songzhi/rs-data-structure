//! [316]去除重复字母
//给你一个仅包含小写字母的字符串，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证返回结果的字典序最小（要求不能打乱其他字符的相对位置）。
//
//
//
// 示例 1:
//
// 输入: "bcabc"
//输出: "abc"
//
//
// 示例 2:
//
// 输入: "cbacdcbc"
//输出: "acdb"
//
//
//
// 注意：该题与 1081 https://leetcode-cn.com/problems/smallest-subsequence-of-distinct
//-characters 相同
// Related Topics 栈 贪心算法

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        use std::collections::{HashMap, HashSet};
        let s = s.into_bytes();
        let mut stack = vec![];
        let mut seen: HashSet<u8> = HashSet::new();
        let last_occurrence = s
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, i))
            .collect::<HashMap<_, _>>();
        for (i, c) in s.into_iter().enumerate() {
            if !seen.contains(&c) {
                while !stack.is_empty()
                    && c.lt(stack.last().unwrap())
                    && i.lt(last_occurrence.get(stack.last().unwrap()).unwrap())
                {
                    seen.remove(&stack.pop().unwrap());
                }
                seen.insert(c);
                stack.push(c);
            }
        }
        unsafe { String::from_utf8_unchecked(stack) }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
