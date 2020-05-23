//! [150]逆波兰表达式求值
//根据逆波兰表示法，求表达式的值。
//
// 有效的运算符包括 +, -, *, / 。每个运算对象可以是整数，也可以是另一个逆波兰表达式。
//
// 说明：
//
//
// 整数除法只保留整数部分。
// 给定逆波兰表达式总是有效的。换句话说，表达式总会得出有效数值且不存在除数为 0 的情况。
//
//
// 示例 1：
//
// 输入: ["2", "1", "+", "3", "*"]
//输出: 9
//解释: ((2 + 1) * 3) = 9
//
//
// 示例 2：
//
// 输入: ["4", "13", "5", "/", "+"]
//输出: 6
//解释: (4 + (13 / 5)) = 6
//
//
// 示例 3：
//
// 输入: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
//输出: 22
//解释:
//  ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
//= ((10 * (6 / (12 * -11))) + 17) + 5
//= ((10 * (6 / -132)) + 17) + 5
//= ((10 * 0) + 17) + 5
//= (0 + 17) + 5
//= 17 + 5
//= 22
// Related Topics 栈

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        use std::ops::{Add, Div, Mul, Sub};
        let mut stack: Vec<i32> = vec![];
        fn operate<F: Fn(i32, i32) -> i32>(stack: &mut Vec<i32>, f: F) {
            let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(f(y, x));
        }
        for token in tokens {
            match token.as_str() {
                "+" => operate(&mut stack, i32::add),
                "-" => operate(&mut stack, i32::sub),
                "*" => operate(&mut stack, i32::mul),
                "/" => operate(&mut stack, i32::div),
                _ => stack.push(token.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
