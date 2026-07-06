//! 移动零

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut left = 0;

        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($input:expr,$expected:expr) => {
            let mut nums = $input;
            Solution::move_zeroes(&mut nums);
            assert_eq!(nums, $expected);
        };
    }

    #[test]
    fn test_move_zeroes() {
        test!(vec![0], vec![0]);
        test!(vec![0, 0, 1], vec![1, 0, 0]);
        test!(vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]);
    }
}
