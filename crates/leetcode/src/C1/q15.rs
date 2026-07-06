//! 三数之和

/// [灵茶山艾府](https://leetcode.cn/problems/3sum/solutions/1968332/shuang-zhi-zhen-xiang-bu-ming-bai-yi-ge-pno55/)
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        let mut ans = vec![];
        nums.sort_unstable();

        for first in 0..(length - 2) {
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }

            let mut third = length - 1;
            let target = -nums[first];

            for second in (first + 1)..(length - 1) {
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue;
                }

                while second < third && nums[second] + nums[third] > target {
                    third -= 1;
                }

                if second == third {
                    break;
                }

                if nums[second] + nums[third] == target {
                    ans.push(vec![nums[first], nums[second], nums[third]]);
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
        test_three_sum{
            Solution::three_sum;

            vec![0, 0, 0] => vec![vec![0, 0, 0]];
            vec![0, 1, 1] => Vec::<Vec<i32>>::new();
            vec![-1, 0, 1, 2, -1, -4] => vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        }
    );
}
