//! 轮转数组

pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let k = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($input:expr,$k:expr,$expected:expr) => {
            let mut input = $input;
            Solution::rotate(&mut input, $k);
            assert_eq!(input, $expected)
        };
    }

    #[test]
    fn test_rotate() {
        test!(vec![1, 2], 3, vec![2, 1]);
        test!(vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]);
    }
}
