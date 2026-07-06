//! 矩阵置零

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let (row, col) = (matrix.len(), matrix[0].len());
        let first_row_has_zero = matrix[0].contains(&0);

        for i in 1..row {
            for j in 0..col {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..row {
            for j in (0..col).rev() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_row_has_zero {
            matrix[0].fill(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($input:expr,$expected:expr) => {
            let mut input = $input;
            Solution::set_zeroes(&mut input);
            assert_eq!(input, $expected);
        };
    }

    /// text
    /// ```
    /// [1,1,1]    [1,0,1]          [0,1,2,0]    [0,0,0,0]
    /// [1,0,1] => [0,0,0]          [3,4,5,2] => [0,4,5,0]
    /// [1,1,1]    [1,0,1]          [1,3,1,5]    [0,3,1,0]
    /// ```
    #[test]
    fn test_set_zeroes() {
        test!(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]
        );

        test!(
            vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
