//! 罗马数字转整数
use std::collections::HashMap;

/// [灵茶山艾府](https://leetcode.cn/problems/roman-to-integer/solutions/2928945/jian-dan-ti-jiu-you-jian-dan-xie-fa-pyth-egyn/)
pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: &str) -> i32 {
        let roman = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);

        let mut ans = 0;
        let s = s.as_bytes();

        for i in 0..(s.len() - 1) {
            let x = roman[&s[i]];
            let y = roman[&s[i + 1]];

            if x < y {
                ans -= x;
            } else {
                ans += x;
            }
        }

        ans + roman[&s[s.len() - 1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_roman_to_int{
            Solution::roman_to_int;

            "III" => 3;
            "IV" => 4;
            "IX" => 9;
            "LVIII" => 58;
            "MCMXCIV" => 1994;
        }
    );
}
