//! 最长连续序列

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut ans = 0;
        for &num in &set {
            if set.contains(&(num - 1)) {
                continue;
            }
            let mut nxt = num + 1;
            while set.contains(&nxt) {
                nxt += 1;
            }
            ans = ans.max(nxt - num);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_longest_consecutive{
            Solution::longest_consecutive;

            vec![0] => 1;
            vec![100, 4, 200, 1, 3, 2] => 4;
            vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1] => 9;
        }
    );
}
