//! 最长递增子序列

pub struct Solution;

impl Solution {
    /// 递归解法
    /// 时间复杂度：O(n^2)
    /// 空间复杂第：O(n)
    pub fn length_of_lis_recursive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut memo = vec![0; n];

        fn dfs(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
            if memo[i] > 0 {
                return memo[i];
            }

            for j in 0..i {
                if nums[j] < nums[i] {
                    memo[i] = memo[i].max(dfs(j, nums, memo));
                }
            }

            memo[i] += 1;
            memo[i]
        }

        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(dfs(i, &nums, &mut memo));
        }

        ans
    }

    /// 动态规划
    /// 时间复杂度：O(n^2)
    /// 空间复杂度：O(n)
    pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j]);
                }
            }

            dp[i] += 1;
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_length_of_lis{
            Solution::length_of_lis_dp;

            vec![0, 1, 0, 3, 2, 3] => 4;
            vec![4, 10, 4, 3, 8, 9] => 3;
            vec![7, 7, 7, 7, 7, 7, 7] => 1;
            vec![10, 9, 2, 5, 3, 7, 101, 18] => 4;
        }
    );
}
