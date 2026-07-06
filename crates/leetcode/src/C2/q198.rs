//! 打家劫舍

pub struct Solution;

impl Solution {
    pub fn rob_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 2];

        for i in 0..n {
            dp[i + 2] = dp[i + 1].max(dp[i] + nums[i]);
        }

        dp[n + 1]
    }

    pub fn rob_optimize(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);

        for num in nums {
            (a, b) = (b, b.max(a + num));
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_rob{
            Solution::rob_dp;
            Solution::rob_optimize;

            vec![1, 2, 3, 1] => 4;
            vec![2, 7, 9, 3, 1] => 12;
        }
    );
}
