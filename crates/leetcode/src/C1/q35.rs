//! 搜索插入位置

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: &[i32], target: i32) -> usize {
        Self::lower_bound(nums, target)
    }

    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
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
        test_search_insert{
            Solution::search_insert;

            &vec![1,3,5,6], 5 => 2;
            &vec![1,3,5,6], 2 => 1;
            &vec![1,3,5,6], 7 => 4;
        }
    );
}
