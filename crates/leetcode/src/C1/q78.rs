//! 子集

/// [灵茶山艾府](https://leetcode.cn/problems/subsets/solutions/2059409/hui-su-bu-hui-xie-tao-lu-zai-ci-pythonja-8tkl/)
pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(i: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            let n = nums.len();

            if i == n {
                ans.push(path.clone());
                return;
            }

            // without choosing
            dfs(i + 1, nums, path, ans);

            // with choosing
            path.push(nums[i]);
            dfs(i + 1, nums, path, ans);
            path.pop();
        }

        let n = nums.len();
        let mut path = Vec::with_capacity(n);
        let mut ans = Vec::with_capacity(1 << n);

        dfs(0, &nums, &mut path, &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_subsets{
            Solution::subsets;

            vec![1,2,3] =>
            vec![vec![],vec![3],vec![2],vec![2,3],vec![1],vec![1,3],vec![1,2],vec![1,2,3]];

            vec![0] =>
            vec![vec![],vec![0]]
        }
    );
}
