//! 完全平方数

const N: usize = 10000;
static mut INITIALIZED: bool = false;
static mut DP: [i32; N + 1] = [i32::MAX >> 1; N + 1];

fn init_once() {
    unsafe {
        if INITIALIZED {
            return;
        }

        INITIALIZED = true;
        DP[0] = 0;

        for i in 1..=100 {
            for j in (i * i)..=N {
                DP[j] = DP[j].min(DP[j - i * i] + 1);
            }
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        unsafe {
            init_once();
            DP[n as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    // text
    // ```
    // 12 = 4 + 4 + 4 => 3
    // 13 = 9 + 4 => 2
    // ```
    test!(
        test_num_squares{
            Solution::num_squares;

            12 => 3;
            13 => 2;
        }
    );
}
