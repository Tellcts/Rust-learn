//! 回文数

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut tmp = x;
        let mut rev = 0;

        while tmp > 0 {
            rev = rev * 10 + tmp % 10;
            tmp /= 10;
        }

        rev == x
    }

    pub fn is_palindrome_optimize(mut x: i32) -> bool {
        if x < 0 || (x > 0 && x % 10 == 0) {
            return false;
        }

        let mut rev = 0;
        while rev < x / 10 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        rev == x || rev == x / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_is_palindrome{
            Solution::is_palindrome;
            Solution::is_palindrome_optimize;

            121 => true;
            -121 => false;
            10 => false;
            12321 => true;
        }
    );
}
