//! [204]计数质数
//统计所有小于非负整数 n 的质数的数量。
//
// 示例:
//
// 输入: 10
//输出: 4
//解释: 小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
//
// Related Topics 哈希表 数学

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut non_prim = std::collections::HashSet::<i32>::new();
        for i in 2..=(n as f32).sqrt() as i32 {
            if !non_prim.contains(&i) {
                for j in (i * i..n).step_by(i as usize) {
                    non_prim.insert(j);
                }
            }
        }

        (n as usize).saturating_sub(non_prim.len() + 2) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
