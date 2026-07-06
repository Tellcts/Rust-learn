//! 合并区间

pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        intervals.sort_unstable_by(|lhs, rhs| lhs[0].cmp(&rhs[0]));

        for p in intervals {
            if let Some(last) = ans.last_mut() {
                if p[0] <= last[1] {
                    last[1] = last[1].max(p[1]);
                } else {
                    ans.push(p);
                }
            } else {
                ans.push(p)
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
        test_merge{
            Solution::merge;

            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]] =>
            vec![vec![1, 6], vec![8, 10], vec![15, 18]];

            vec![vec![1, 4], vec![4, 5]] => vec![vec![1, 5]];
        }
    );
}
