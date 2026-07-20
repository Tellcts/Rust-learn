//! 在排序数组中查找元素的第一个和最后一个位置

/// [灵茶山艾府](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/1980196/er-fen-cha-zhao-zong-shi-xie-bu-dui-yi-g-t9l9/)
pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = Self::lower_bound(&nums, target);

        if start == nums.len() || nums[start] != target {
            return vec![-1, -1];
        }

        let end = Self::lower_bound(&nums, target + 1) - 1;

        vec![start as _, end as _]
    }

    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_search_range{
            Solution::search_range;

            vec![], 0 => vec![-1,-1];
            vec![5,7,7,8,8,10], 8 => vec![3,4];
            vec![5,7,7,8,8,10], 6 => vec![-1,-1];
        }
    );
}
