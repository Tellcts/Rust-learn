//! 有效的数独

/// [灵茶山艾府](https://leetcode.cn/problems/valid-sudoku/solutions/3767330/ji-lu-mei-xing-mei-lie-mei-gong-zhong-de-qa0x/)
pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: &[Vec<u8>]) -> bool {
        let mut row_has = vec![[false; 9]; 9];
        let mut col_has = vec![[false; 9]; 9];
        let mut sub_has = vec![[[false; 9]; 3]; 3];

        for (i, row) in board.iter().enumerate() {
            for (j, &b) in row.iter().enumerate() {
                if b == b'.' {
                    continue;
                }

                let idx = (b - b'1') as usize;

                if row_has[i][idx] || col_has[j][idx] || sub_has[i / 3][j / 3][idx] {
                    return false;
                }

                row_has[i][idx] = true;
                col_has[j][idx] = true;
                sub_has[i / 3][j / 3][idx] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_is_valid_sudoku{
            Solution::is_valid_sudoku;

            &[vec![b'5',b'3',b'.',b'.',b'7',b'.',b'.',b'.',b'.']
             ,vec![b'6',b'.',b'.',b'1',b'9',b'5',b'.',b'.',b'.']
             ,vec![b'.',b'9',b'8',b'.',b'.',b'.',b'.',b'6',b'.']
             ,vec![b'8',b'.',b'.',b'.',b'6',b'.',b'.',b'.',b'3']
             ,vec![b'4',b'.',b'.',b'8',b'.',b'3',b'.',b'.',b'1']
             ,vec![b'7',b'.',b'.',b'.',b'2',b'.',b'.',b'.',b'6']
             ,vec![b'.',b'6',b'.',b'.',b'.',b'.',b'2',b'8',b'.']
             ,vec![b'.',b'.',b'.',b'4',b'1',b'9',b'.',b'.',b'5']
             ,vec![b'.',b'.',b'.',b'.',b'8',b'.',b'.',b'7',b'9']] => true;

            &[vec![b'8',b'3',b'.',b'.',b'7',b'.',b'.',b'.',b'.']
             ,vec![b'6',b'.',b'.',b'1',b'9',b'5',b'.',b'.',b'.']
             ,vec![b'.',b'9',b'8',b'.',b'.',b'.',b'.',b'6',b'.']
             ,vec![b'8',b'.',b'.',b'.',b'6',b'.',b'.',b'.',b'3']
             ,vec![b'4',b'.',b'.',b'8',b'.',b'3',b'.',b'.',b'1']
             ,vec![b'7',b'.',b'.',b'.',b'2',b'.',b'.',b'.',b'6']
             ,vec![b'.',b'6',b'.',b'.',b'.',b'.',b'2',b'8',b'.']
             ,vec![b'.',b'.',b'.',b'4',b'1',b'9',b'.',b'.',b'5']
             ,vec![b'.',b'.',b'.',b'.',b'8',b'.',b'.',b'7',b'9']] => false;
        }
    );
}
