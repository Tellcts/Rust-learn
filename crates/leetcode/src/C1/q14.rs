//! 最长公共前缀

/// [灵茶山艾府](https://leetcode.cn/problems/longest-common-prefix/solutions/2801713/jian-dan-ti-jian-dan-zuo-pythonjavaccgoj-478q/)
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix<T>(strs: Vec<T>) -> String
    where
        T: AsRef<str>,
    {
        let s0 = strs[0].as_ref();

        for (col, ch) in s0.bytes().enumerate() {
            for str in &strs[1..] {
                let str = str.as_ref();

                if col == str.len() || str.as_bytes()[col] != ch {
                    return s0[..col].to_string();
                }
            }
        }

        s0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_longest_common_prefix{
            Solution::longest_common_prefix;

            vec![""] => "".to_string();
            vec!["dog", "racecar", "car"] => "".to_string();
            vec!["flower", "flow", "flight"] => "fl".to_string();
        }
    );
}
