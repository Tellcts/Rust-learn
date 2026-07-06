//! 和为K的子数组

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut ans, mut pre_sum) = (0, 0);
        let mut cnt = HashMap::from([(0, 1)]);

        for num in nums {
            pre_sum += num;
            ans += cnt.get(&(pre_sum - k)).copied().unwrap_or(0);
            *cnt.entry(pre_sum).or_insert(0) += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_subarray_sum{
            Solution::subarray_sum;

            vec![1, 1, 1], 2 => 2;
            vec![1, 2, 3], 3 => 2;
        }
    );
}
