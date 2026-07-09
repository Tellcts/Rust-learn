//! 下一个排列

/// [灵茶山艾府](https://leetcode.cn/problems/next-permutation/solutions/3621022/jiao-ni-cong-ling-kai-shi-si-kao-zhe-ti-9qfrq/)
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        let n = nums.len();
        let mut idx = n as isize - 2;

        while idx >= 0 && nums[idx as usize] >= nums[(idx + 1) as usize] {
            idx -= 1;
        }

        if idx >= 0 {
            let i = idx as usize;
            let mut j = n - 1;

            while nums[j] <= nums[i] {
                j -= 1;
            }

            nums.swap(i, j);
        }

        nums[(idx + 1) as usize..].reverse();
    }

    pub fn pre_permutation(nums: &mut [i32]) {
        let n = nums.len();
        let mut idx = n as isize - 2;

        while idx >= 0 && nums[idx as usize] <= nums[(idx + 1) as usize] {
            idx -= 1;
        }

        if idx >= 0 {
            let i = idx as usize;
            let mut j = n - 1;

            while nums[j] >= nums[i] {
                j -= 1;
            }

            nums.swap(i, j);
        }

        nums[(idx + 1) as usize..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_next {
        ($input:expr,$expected:expr) => {
            let mut input = $input;
            let expected = $expected;

            Solution::next_permutation(&mut input);
            assert_eq!(input, expected);
        };
    }

    macro_rules! test_pre {
        ($input:expr,$expected:expr) => {
            let mut input = $input;
            let expected = $expected;

            Solution::pre_permutation(&mut input);
            assert_eq!(input, expected);
        };
    }

    #[test]
    fn test_next_permutation() {
        test_next!([1, 2, 3], [1, 3, 2]);
        test_next!([3, 2, 1], [1, 2, 3]);
        test_next!([1, 1, 5], [1, 5, 1]);
    }

    #[test]
    fn test_pre_permutation() {
        test_pre!([1, 2, 3], [3, 2, 1]);
        test_pre!([3, 2, 1], [3, 1, 2]);
        test_pre!([1, 5, 1], [1, 1, 5]);
    }
}
