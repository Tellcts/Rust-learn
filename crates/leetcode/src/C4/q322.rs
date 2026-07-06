//! 零钱兑换

pub struct Solution;

impl Solution {
    pub fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
        let (n, amount) = (coins.len(), amount as usize);
        let mut dp = vec![vec![i32::MAX >> 1; amount + 1]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for j in 0..=amount {
                if j < coins[i] as usize {
                    dp[i + 1][j] = dp[i][j];
                } else {
                    dp[i + 1][j] = dp[i][j].min(dp[i + 1][j - coins[i] as usize] + 1);
                }
            }
        }

        let ans = dp[n][amount];
        if ans != (i32::MAX >> 1) { ans } else { -1 }
    }

    pub fn coin_change_optimize(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX >> 1; amount + 1];
        dp[0] = 0;

        for coin in coins {
            for i in (coin as usize)..=amount {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }

        let ans = dp[amount];
        if ans != (i32::MAX >> 1) { ans } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_coin_change{
            Solution::coin_change_dp;
            Solution::coin_change_optimize;

            vec![], 0 => 0;
            vec![1], 0 => 0;
            vec![2], 3 => -1;
            vec![1, 2, 5], 11 => 3;
        }
    );
}
