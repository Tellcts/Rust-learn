//! 字符串转换整数（atoi）

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut idx = 0;

        // 跳过前导空格
        while idx < n && s[idx] == b' ' {
            idx += 1;
        }

        // 处理符号
        let mut sign = 1;
        if idx < n && (s[idx] == b'+' || s[idx] == b'-') {
            if s[idx] == b'-' {
                sign = -1;
            }

            idx += 1;
        }

        // 处理数字
        let mut ans = 0;
        while idx < n && s[idx].is_ascii_digit() {
            let digit = (s[idx] - b'0') as i32;

            if ans > i32::MAX / 10 || ans * 10 > i32::MAX - digit {
                return if sign > 0 { i32::MAX } else { i32::MIN };
            }

            ans = ans * 10 + digit;
            idx += 1;
        }

        sign * ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_my_atoi{
            Solution::my_atoi;

            "42".to_string() => 42;
            "   -42".to_string() => -42;
            "-042".to_string() => -42;
            "4193 with words".to_string() => 4193;
            "words and 987".to_string() => 0;
            "1337c0d3".to_string() => 1337;
        }
    );
}
