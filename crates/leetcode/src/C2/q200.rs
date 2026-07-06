//! 岛屿数量

pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
                return;
            }

            grid[i][j] = '2';
            dfs(grid, i, j + 1);
            dfs(grid, i + 1, j);
            // 如果 i,j 已经是零，那么模环绕会使得dfs函数提前返回
            dfs(grid, i.wrapping_sub(1), j);
            dfs(grid, i, j.wrapping_sub(1));
        }

        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    ans += 1;
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
        test_num_islands{
            Solution::num_islands;

            vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '1']] => 3
        }
    );
}
