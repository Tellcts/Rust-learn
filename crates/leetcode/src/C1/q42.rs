//! 接雨水

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut left_max, mut right_max) = (0, 0);
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);

            if left_max < right_max {
                ans += left_max - height[left];
                left += 1;
            } else {
                ans += right_max - height[right];
                right -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_trap{
            Solution::trap;

            vec![4, 2, 0, 3, 2, 5] => 9;
            vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1] => 6;
        }
    );
}
