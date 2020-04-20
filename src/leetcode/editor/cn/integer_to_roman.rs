//! [12]整数转罗马数字
//罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
//
// 字符          数值
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
//
// 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做 XXVII, 即为 XX + V + I
//I 。
//
// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5
// 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
//
//
// I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
// X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
// C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
//
//
// 给定一个整数，将其转为罗马数字。输入确保在 1 到 3999 的范围内。
//
// 示例 1:
//
// 输入: 3
//输出: "III"
//
// 示例 2:
//
// 输入: 4
//输出: "IV"
//
// 示例 3:
//
// 输入: 9
//输出: "IX"
//
// 示例 4:
//
// 输入: 58
//输出: "LVIII"
//解释: L = 50, V = 5, III = 3.
//
//
// 示例 5:
//
// 输入: 1994
//输出: "MCMXCIV"
//解释: M = 1000, CM = 900, XC = 90, IV = 4.
// Related Topics 数学 字符串

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = vec![];
        while num > 0 {
            match num {
                1..=3 => {
                    result.push(b'I');
                    num -= 1;
                }
                4 => {
                    result.extend_from_slice(b"IV");
                    num -= 4;
                }
                5..=8 => {
                    result.push(b'V');
                    num -= 5;
                }
                9 => {
                    result.extend_from_slice(b"IX");
                    num -= 9;
                }
                10..=39 => {
                    result.push(b'X');
                    num -= 10;
                }
                40..=49 => {
                    result.extend_from_slice(b"XL");
                    num -= 40;
                }
                50..=89 => {
                    result.push(b'L');
                    num -= 50;
                }
                90..=99 => {
                    result.extend_from_slice(b"XC");
                    num -= 90;
                }
                100..=399 => {
                    result.push(b'C');
                    num -= 100;
                }
                400..=499 => {
                    result.extend_from_slice(b"CD");
                    num -= 400;
                }
                500..=899 => {
                    result.push(b'D');
                    num -= 500;
                }
                900..=999 => {
                    result.extend_from_slice(b"CM");
                    num -= 900;
                }
                _ => {
                    result.push(b'M');
                    num -= 1000;
                }
            }
        }
        // fn deal_special_num(num: i32) -> Option<(&'static [u8], i32)> {
        //     match num {
        //         4 => Some((b"IV", num - 4)),
        //         9 => Some((b"IX", num - 9)),
        //         40..=49 => Some((b"XL", num - 40)),
        //         90..=99 => Some((b"XC", num - 90)),
        //         400..=499 => Some((b"CD", num - 400)),
        //         900..=999 => Some((b"CM", num - 900)),
        //         _ => None,
        //     }
        // }
        // let numbers = [1, 5, 10, 50, 100, 500, 1000];
        // let chars = b"IVXLCDM".to_owned();
        // let mut result: Vec<u8> = vec![];
        // for (i, &n) in numbers.iter().enumerate().rev() {
        //     if let Some((chars, rev)) = deal_special_num(num) {
        //         result.extend_from_slice(chars);
        //         num = rev;
        //     } else if num >= n {
        //         for _ in 0..num / n {
        //             result.push(chars[i]);
        //         }
        //         num %= n;
        //     }
        // }
        unsafe { String::from_utf8_unchecked(result) }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
