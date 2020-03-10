//! [322]零钱兑换
//给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回
// -1。
//
// 示例 1:
//
// 输入: coins = [1, 2, 5], amount = 11
//输出: 3
//解释: 11 = 5 + 5 + 1
//
// 示例 2:
//
// 输入: coins = [2], amount = 3
//输出: -1
//
// 说明:
//你可以认为每种硬币的数量是无限的。
// Related Topics 动态规划

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::ops::Add;
        use std::ops::Sub;
        let mut dp = vec![-1i32; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            dp[i as usize] = coins
                .iter()
                .map(|coin| dp.get(i.sub(coin) as usize).filter(|&c| c.ne(&-1)))
                .flatten()
                .min()
                .map(|c| c.add(&1))
                .unwrap_or(-1);
        }
        dp.pop().unwrap_or(-1)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
