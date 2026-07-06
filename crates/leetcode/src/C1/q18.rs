//! 四数之和

/// [灵茶山艾府](https://leetcode.cn/problems/4sum/solutions/2344514/ji-zhi-you-hua-ji-yu-san-shu-zhi-he-de-z-1f0b/)
pub struct Solution;

impl Solution {
    #[rustfmt::skip]
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let target = target as i64;
        let mut ans = vec![];

        for first in 0..n.saturating_sub(3) {
            // Skip the repeated numbers
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }

            // Optimization Point
            if (nums[first] as i64
                + nums[first + 1] as i64
                + nums[first + 2] as i64
                + nums[first + 3] as i64)
                > target
            {
                break;
            }

            // Optimization Point
            if (nums[first] as i64 
                + nums[n - 3] as i64 
                + nums[n - 2] as i64 
                + nums[n - 1] as i64)
                < target
            {
                continue;
            }

            for second in (first + 1)..(n - 2) {
                // Skip the repeated numbers
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue;
                }

                // Optimization Point
                if (nums[first] as i64
                    + nums[second] as i64
                    + nums[second + 1] as i64
                    + nums[second + 2] as i64)
                    > target
                {
                    break;
                }

                // Optimization Point
                if (nums[first] as i64
                    + nums[second] as i64
                    + nums[n - 2] as i64
                    + nums[n - 1] as i64)
                    < target
                {
                    continue;
                }

                let (mut third, mut forth) = (second + 1, n - 1);
                while third < forth {
                    let sum = nums[first] as i64
                        + nums[second] as i64
                        + nums[third] as i64
                        + nums[forth] as i64;

                    if sum > target {
                        forth -= 1;
                    } else if sum < target {
                        third += 1;
                    } else {    // sum == target
                        ans.push(vec![nums[first], nums[second], nums[third], nums[forth]]);

                        third += 1;
                        // Skip the repeated numbers
                        while third < forth && nums[third] == nums[third - 1] {
                            third += 1;
                        }

                        forth -= 1;
                        // Skip the repeated numbers
                        while forth > third && nums[forth] == nums[forth + 1] {
                            forth -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::test;

    test!(
        test_four_sum{
            Solution::four_sum;

            vec![2,2,2,2,2], 8 => vec![vec![2,2,2,2]];
            vec![1,0,-1,0,-2,2], 0 => vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]];
        }
    );
}


