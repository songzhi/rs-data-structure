//! [385]迷你语法分析器
//给定一个用字符串表示的整数的嵌套列表，实现一个解析它的语法分析器。
//
// 列表中的每个元素只可能是整数或整数嵌套列表
//
// 提示：你可以假定这些字符串都是格式良好的：
//
//
// 字符串非空
// 字符串不包含空格
// 字符串只包含数字0-9, [ - , ]
//
//
//
//
// 示例 1：
//
//
//给定 s = "324",
//
//你应该返回一个 NestedInteger 对象，其中只包含整数值 324。
//
//
//
//
// 示例 2：
//
//
//给定 s = "[123,[456,[789]]]",
//
//返回一个 NestedInteger 对象包含一个有两个元素的嵌套列表：
//
//1. 一个 integer 包含值 123
//2. 一个包含两个元素的嵌套列表：
//    i.  一个 integer 包含值 456
//    ii. 一个包含一个元素的嵌套列表
//         a. 一个 integer 包含值 789
//
//
//
// Related Topics 栈 字符串

//leetcode submit region begin(Prohibit modification and deletion)
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn parse_number(chars: &mut Peekable<Chars>, mut n: u32) -> i32 {
    while let Some(&c) = chars.peek() {
        if !c.is_numeric() {
            break;
        }
        n = n * 10 + c.to_digit(10).unwrap();
        chars.next();
    }
    n as i32
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut chars = s.chars().peekable();
        let mut stack: Vec<NestedInteger> = vec![];
        while let Some(c) = chars.next() {
            match c {
                '[' => stack.push(NestedInteger::List(vec![])),
                '-' | '0'..='9' => {
                    let n = if c == '-' {
                        -parse_number(&mut chars, 0)
                    } else {
                        parse_number(&mut chars, c.to_digit(10).unwrap())
                    };
                    if let Some(NestedInteger::List(mut items)) = stack.pop() {
                        items.push(NestedInteger::Int(n));
                        stack.push(NestedInteger::List(items));
                    } else {
                        stack.push(NestedInteger::Int(n));
                    }
                }
                ']' => {
                    if stack.len() >= 2 {
                        let (top, second) = (stack.pop().unwrap(), stack.pop().unwrap());
                        if let NestedInteger::List(mut items) = second {
                            items.push(top);
                            stack.push(NestedInteger::List(items));
                        } else {
                            stack.push(second);
                            stack.push(top);
                        }
                    }
                }
                _ => continue,
            }
        }
        stack.pop().unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
