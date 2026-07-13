//! 搜索旋转排序数组

/// [灵茶山艾府](https://leetcode.cn/problems/search-in-rotated-sorted-array/solutions/1987503/by-endlesscheng-auuh/)
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let min_idx = Self::find_min(&nums);

        if target > nums[nums.len() - 1] {
            Self::lower_bound(&nums, 0, min_idx, target)
        } else {
            Self::lower_bound(&nums, min_idx, nums.len(), target)
        }
    }

    fn find_min(nums: &[i32]) -> usize {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < nums[nums.len() - 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn lower_bound(nums: &[i32], mut left: usize, mut right: usize, target: i32) -> i32 {
        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if nums[left] == target { left as _ } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_search{
            Solution::search;

            vec![1], 0 => -1;
            vec![4,5,6,7,0,1,2], 0 => 4;
            vec![4,5,6,7,0,1,2], 3 => -1;
        }
    );
}
