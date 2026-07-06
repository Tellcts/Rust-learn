//! 最接近的三数之和

/// [灵茶山艾府](https://leetcode.cn/problems/3sum-closest/solutions/2337801/ji-zhi-you-hua-ji-yu-san-shu-zhi-he-de-z-qgqi/)
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = i32::MAX / 2;

        for first in 0..(n - 2) {
            // optimize: skip duplicate numbers
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }

            // optimize: early termination
            let mut sum = nums[first] + nums[first + 1] + nums[first + 2];
            if sum > target {
                if sum - target < (ans - target).abs() {
                    ans = sum;
                }

                break;
            }

            // optimize: early termination
            sum = nums[first] + nums[n - 2] + nums[n - 1];
            if sum < target {
                if target - sum < (ans - target).abs() {
                    ans = sum;
                }

                continue;
            }

            let mut second = first + 1;
            let mut third = n - 1;

            while second < third {
                sum = nums[first] + nums[second] + nums[third];

                if sum == target {
                    return target;
                }

                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }

                if sum > target {
                    third -= 1;
                } else {
                    second += 1;
                }
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
        test_three_sum_closest{
            Solution::three_sum_closest;

            vec![0, 0, 0], 1 => 0;
            vec![-1, 2, 1, -4], 1 => 2;
        }
    );
}
