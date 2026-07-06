//! 旋转图像

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let n = matrix.len();

        for i in 0..n {
            for j in (i + 1)..n {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }

            matrix[i].reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($input:expr,$expected:expr) => {
            let mut origin = $input;
            Solution::rotate(&mut origin);
            assert_eq!(origin, $expected);
        };
    }

    /// text
    /// ```
    /// [1,2,3]        [7,4,1]          
    /// [4,5,6]   =>   [8,5,2]
    /// [7,8,9]        [9,6,3]
    ///
    /// [ 5, 1, 9,11]        [15,13, 2, 5]
    /// [ 2, 4, 8,10]   =>   [14, 3, 4, 1]
    /// [13, 3, 6, 7]        [12, 6, 8, 9]
    /// [15,14,12,16]        [16, 7,10,11]
    /// ```
    #[test]
    fn test_rotate() {
        test!(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );

        test!(
            vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16]
            ],
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
