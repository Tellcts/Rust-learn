//! 移除元素

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], target: i32) -> usize {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != target {
                nums[k] = nums[i];
                k += 1;
            } else {
                continue;
            }
        }

        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_remove_element{
            Solution::remove_element;

            &mut [3,2,2,3], 3 => 2;
            &mut [0,1,2,2,3,0,4,2], 2 => 5;
        }
    );
}
