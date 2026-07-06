//! 分割等和子集

pub struct Solution;

impl Solution {
    pub fn can_partition_recursive(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if !sum.is_multiple_of(2) {
            return false;
        }

        fn dfs(i: usize, j: usize, nums: &[i32], memo: &mut [Vec<i32>]) -> bool {
            if i == nums.len() {
                return j == 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j] == 1;
            }

            let x = nums[i] as usize;
            let res = if j < x {
                dfs(i + 1, j, nums, memo)
            } else {
                dfs(i + 1, j - x, nums, memo) || dfs(i + 1, j, nums, memo)
            };

            memo[i][j] = if res { 1 } else { 0 };

            res
        }

        let n = nums.len();
        let mut memo = vec![vec![-1; sum / 2 + 1]; n];
        dfs(0, sum / 2, &nums, &mut memo)
    }

    pub fn can_partition_dp(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if !sum.is_multiple_of(2) {
            return false;
        }

        let (s, n) = (sum / 2, nums.len());
        let mut dp = vec![vec![false; s + 1]; n + 1];
        dp[0][0] = true;

        for (i, &num) in nums.iter().enumerate() {
            let num = num as usize;
            for j in 0..=s {
                dp[i + 1][j] = (j >= num && dp[i][j - num]) || dp[i][j];
            }
        }

        dp[n][s]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_can_partition{
            Solution::can_partition_dp;
            Solution::can_partition_recursive;

            vec![1, 2, 3, 5] => false;
            vec![1, 5, 11, 5] => true;
        }
    );
}
