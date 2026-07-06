//! 腐烂的橘子

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let (mut fresh, mut rotten) = (0, vec![]);

        for (i, rows) in grid.iter().enumerate() {
            for (j, &x) in rows.iter().enumerate() {
                if x == 1 {
                    fresh += 1;
                } else if x == 2 {
                    rotten.push((i, j));
                }
            }
        }

        let mut ans = 0;

        while fresh > 0 && !rotten.is_empty() {
            ans += 1;
            let mut new_rotten = vec![];

            for (x, y) in rotten {
                for (i, j) in [
                    (x + 1, y),
                    (x, y + 1),
                    (x.wrapping_sub(1), y),
                    (x, y.wrapping_sub(1)),
                ] {
                    if i < row && j < col && grid[i][j] == 1 {
                        fresh -= 1;
                        grid[i][j] = 2;
                        new_rotten.push((i, j));
                    }
                }
            }

            rotten = new_rotten;
        }

        if fresh == 0 { ans } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    // text
    // ```
    // [2,1,1]
    // [1,1,0]   =>   4
    // [0,1,1]
    //
    // [2,1,1]
    // [0,1,1]   =>   -1
    // [1,0,1]
    // ```
    test!(
       test_oranges_rotting{
           Solution::oranges_rotting;

           vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]] => 4;
           vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]] => -1;
    }
    );
}
