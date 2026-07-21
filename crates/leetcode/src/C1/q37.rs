//! 解数独

use std::collections::BinaryHeap;

/// [灵茶山艾府](https://leetcode.cn/problems/sudoku-solver/solutions/3767438/shu-du-zen-yao-wan-ti-mu-jiu-zen-yao-zuo-ms2q/)
pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut [Vec<u8>]) {
        let mut row_has = [[false; 9]; 9];
        let mut col_has = [[false; 9]; 9];
        let mut sub_has = [[[false; 9]; 3]; 3];
        let mut empty_pos = vec![];

        for (i, row) in board.iter().enumerate() {
            for (j, &b) in row.iter().enumerate() {
                if b == b'.' {
                    empty_pos.push((i, j));
                } else {
                    let idx = (b - b'1') as usize;

                    row_has[i][idx] = true;
                    col_has[j][idx] = true;
                    sub_has[i / 3][j / 3][idx] = true;
                }
            }
        }

        let get_candidates = |i: usize, j: usize| -> i8 {
            let mut candidates = 9;

            for idx in 0..9 {
                if row_has[i][idx] || col_has[j][idx] || sub_has[i / 3][j / 3][idx] {
                    candidates -= 1;
                }
            }

            candidates
        };

        let mut empty_heap = BinaryHeap::new();

        for (i, j) in empty_pos {
            empty_heap.push((-get_candidates(i, j), i, j));
        }

        Self::dfs(
            board,
            &mut row_has,
            &mut col_has,
            &mut sub_has,
            &mut empty_heap,
        );
    }

    fn dfs(
        board: &mut [Vec<u8>],
        row_has: &mut [[bool; 9]; 9],
        col_has: &mut [[bool; 9]; 9],
        sub_has: &mut [[[bool; 9]; 3]; 3],
        empty_heap: &mut BinaryHeap<(i8, usize, usize)>,
    ) -> bool {
        if empty_heap.is_empty() {
            return true;
        }

        let mut candidates = 0;
        let (_, i, j) = empty_heap.pop().unwrap();

        for idx in 0..9 {
            if row_has[i][idx] || col_has[j][idx] || sub_has[i / 3][j / 3][idx] {
                continue;
            }

            board[i][j] = b'1' + idx as u8;

            row_has[i][idx] = true;
            col_has[j][idx] = true;
            sub_has[i / 3][j / 3][idx] = true;

            if Self::dfs(board, row_has, col_has, sub_has, empty_heap) {
                return true;
            }

            row_has[i][idx] = false;
            col_has[j][idx] = false;
            sub_has[i / 3][j / 3][idx] = false;

            candidates += 1;
        }

        empty_heap.push((-candidates, i, j));

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut board = [
            vec![b'5', b'3', b'.', b'.', b'7', b'.', b'.', b'.', b'.'],
            vec![b'6', b'.', b'.', b'1', b'9', b'5', b'.', b'.', b'.'],
            vec![b'.', b'9', b'8', b'.', b'.', b'.', b'.', b'6', b'.'],
            vec![b'8', b'.', b'.', b'.', b'6', b'.', b'.', b'.', b'3'],
            vec![b'4', b'.', b'.', b'8', b'.', b'3', b'.', b'.', b'1'],
            vec![b'7', b'.', b'.', b'.', b'2', b'.', b'.', b'.', b'6'],
            vec![b'.', b'6', b'.', b'.', b'.', b'.', b'2', b'8', b'.'],
            vec![b'.', b'.', b'.', b'4', b'1', b'9', b'.', b'.', b'5'],
            vec![b'.', b'.', b'.', b'.', b'8', b'.', b'.', b'7', b'9'],
        ];

        let expected = [
            vec![b'5', b'3', b'4', b'6', b'7', b'8', b'9', b'1', b'2'],
            vec![b'6', b'7', b'2', b'1', b'9', b'5', b'3', b'4', b'8'],
            vec![b'1', b'9', b'8', b'3', b'4', b'2', b'5', b'6', b'7'],
            vec![b'8', b'5', b'9', b'7', b'6', b'1', b'4', b'2', b'3'],
            vec![b'4', b'2', b'6', b'8', b'5', b'3', b'7', b'9', b'1'],
            vec![b'7', b'1', b'3', b'9', b'2', b'4', b'8', b'5', b'6'],
            vec![b'9', b'6', b'1', b'5', b'3', b'7', b'2', b'8', b'4'],
            vec![b'2', b'8', b'7', b'4', b'1', b'9', b'6', b'3', b'5'],
            vec![b'3', b'4', b'5', b'2', b'8', b'6', b'1', b'7', b'9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(board, expected);
    }
}
