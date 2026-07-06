//! 乘积最大子数组

pub struct Solution;

impl Solution {
    pub fn max_product_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_max = vec![0; n];
        let mut dp_min = vec![0; n];
        (dp_max[0], dp_min[0]) = (nums[0], nums[0]);

        for i in 1..n {
            (dp_max[i], dp_min[i]) = (
                nums[i]
                    .max(nums[i] * dp_max[i - 1])
                    .max(nums[i] * dp_min[i - 1]),
                nums[i]
                    .min(nums[i] * dp_max[i - 1])
                    .min(nums[i] * dp_min[i - 1]),
            );
        }

        dp_max.into_iter().max().unwrap()
    }

    pub fn max_product_optimize(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let (mut f_max, mut f_min) = (1, 1);

        for num in nums {
            (f_max, f_min) = (
                num.max(f_max * num).max(f_min * num),
                num.min(f_max * num).min(f_min * num),
            );

            ans = ans.max(f_max);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_max_product{
            Solution::max_product_dp;
            Solution::max_product_optimize;

            vec![-2, 0, -1] => 0;
            vec![2, 3, -2, 4] => 6;
        }
    );
}
