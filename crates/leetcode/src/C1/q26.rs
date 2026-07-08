//! 删除有序数组中的重复项

/// [灵茶山艾府](https://leetcode.cn/problems/remove-duplicates-from-sorted-array/solutions/2807162/gen-zhao-wo-guo-yi-bian-shi-li-2ni-jiu-m-rvyk/)
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> usize {
        let mut k = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[k] = nums[i];
                k += 1;
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
        test_remove_duplicates{
            Solution::remove_duplicates;

           &mut [1,1,2] => 2;
           &mut [0,0,1,1,1,2,2,3,3,4] => 5;
        }
    );
}
