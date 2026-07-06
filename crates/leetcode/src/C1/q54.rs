//! 螺旋矩阵

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // (i,j)分别表示在行号、列号上的增量
        // Right:(0,1) Down:(1,0) Left:(0,-1) Up:(-1,0)
        const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let (row, col) = (matrix.len(), matrix[0].len());
        let mut ans = vec![0; row * col];
        let (mut i, mut j, mut direction) = (0, 0, 0);

        for k in 0..(row * col) {
            ans[k] = matrix[i][j];
            matrix[i][j] = i32::MAX;
            let (next_i, next_j) = (
                (i as isize + DIRS[direction].0),
                (j as isize + DIRS[direction].1),
            );

            if next_j < 0
                || next_j >= col as isize
                || next_i >= row as isize
                || matrix[next_i as usize][next_j as usize] == i32::MAX
            {
                direction = (direction + 1) % 4;
            }

            i = (i as isize + DIRS[direction].0) as usize;
            j = (j as isize + DIRS[direction].1) as usize;
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
    // [1,2,3]
    // [4,5,6]   =>   [1,2,3,6,9,8,7,4,5]
    // [7,8,9]
    // ```
    test!(
        test_spiral_order{
            Solution::spiral_order;

            vec![vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]] => vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        }
    );
}
