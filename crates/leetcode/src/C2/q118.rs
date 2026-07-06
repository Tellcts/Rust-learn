//! 杨辉三角

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut ans = vec![vec![]; n];

        for i in 0..n {
            ans[i].resize(i + 1, 1);

            for j in 1..i {
                ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    // text
    // ```
    //        1
    //      1   1
    //    1   2   1
    //  1   3   3   1
    //1   4   6   4   1
    // ```
    test!(
        test_generate{
            Solution::generate;

            5 => vec![
                 vec![1],
                 vec![1, 1],
                 vec![1, 2, 1],
                 vec![1, 3, 3, 1],
                 vec![1, 4, 6, 4, 1]
                ]
        }
    );
}
