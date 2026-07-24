//! 组合总和

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut path = vec![];

        Self::dfs(0, target, candidates, &mut ans, &mut path);

        ans
    }

    fn dfs(
        idx: usize,
        left: i32,
        candidates: &[i32],
        ans: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if left == 0 {
            ans.push(path.to_vec());
            return;
        }

        if idx == candidates.len() || left < 0 {
            return;
        }

        Self::dfs(idx + 1, left, candidates, ans, path);

        path.push(candidates[idx]);
        Self::dfs(idx, left - candidates[idx], candidates, ans, path);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_combination_sum{
            Solution::combination_sum;

            &[2,3,6,7], 7 => vec![vec![7], vec![2,2,3]];
            &[2,3,5], 8 => vec![vec![3,5], vec![2,3,3], vec![2,2,2,2]];
        }
    );
}
