//! 两数之和

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 暴力解法
    /// 时间复杂度：O(n^2)
    /// 空间复杂度：O(1)
    pub fn two_sum_violent(nums: Vec<i32>, target: i32) -> Vec<usize> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i, j];
                }
            }
        }
        // 消除编译错误：明确 for 循环会提前返回，否则直接崩溃
        unreachable!()
    }

    /// 时间复杂度：O(n)
    /// 空间复杂度：O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
        let mut idx = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = idx.get(&(target - num)) {
                return vec![j, i];
            }

            idx.insert(num, i);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
            test_two_sum{
            Solution::two_sum;
            Solution::two_sum_violent;

            vec![3,3], 6 => vec![0,1];
            vec![3,2,4], 6 => vec![1,2];
            vec![2,7,11,15], 9 => vec![0,1];
        }
    );
}
