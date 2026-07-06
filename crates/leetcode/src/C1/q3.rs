//! 无重复字符的最长子串

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: &str) -> usize {
        let s = s.as_bytes();
        let mut cnt = [0; 128];
        let (mut ans, mut left) = (0, 0);

        for (right, &ch) in s.iter().enumerate() {
            let ch = ch as usize;
            cnt[ch] += 1;

            while cnt[ch] > 1 {
                cnt[s[left] as usize] -= 1;
                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_length_of_longest_substring{
            Solution::length_of_longest_substring;

            "bbbbb" => 1;
            "pwwkew" => 3;
            "abcabcbb" => 3;
        }
    );
}
