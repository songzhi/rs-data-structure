//! [224]基本计算器
//实现一个基本的计算器来计算一个简单的字符串表达式的值。 
//
// 字符串表达式可以包含左括号 ( ，右括号 )，加号 + ，减号 -，非负整数和空格 。 
//
// 示例 1: 
//
// 输入: "1 + 1"
//输出: 2
// 
//
// 示例 2: 
//
// 输入: " 2-1 + 2 "
//输出: 3 
//
// 示例 3: 
//
// 输入: "(1+(4+5+2)-3)+(6+8)"
//输出: 23 
//
// 说明： 
//
// 
// 你可以假设所给定的表达式都是有效的。 
// 请不要使用内置的库函数 eval。 
// 
// Related Topics 栈 数学


pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = vec![];
        let mut operand = 0;
        let mut result = 0;
        let mut sign = 1;
        for c in s.chars() {
            match c {
                '0'..='9' => operand = operand * 10 + c.to_digit(10).unwrap(),
                '+' => {
                    result += sign * (operand as i32);
                    sign = 1;
                    operand = 0;
                }
                '-' => {
                    result += sign * (operand as i32);
                    sign = -1;
                    operand = 0;
                }
                '(' => {
                    stack.push(result);
                    stack.push(sign);
                    sign = 1;
                    result = 0;
                }
                ')' => {
                    result += sign * (operand as i32);
                    result *= stack.pop().unwrap();
                    result += stack.pop().unwrap();
                    operand = 0;
                }
                _ => continue
            }
        }
        result + sign * (operand as i32)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
