//! 电话号码的字母组合

/// [灵茶山艾府](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/solutions/2059416/hui-su-bu-hui-xie-tao-lu-zai-ci-pythonja-3orv/)
pub struct Solution;

impl Solution {
    const MAPPING: [&str; 10] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let n = digits.len();

        if n == 0 {
            return vec![];
        }

        fn dfs(i: usize, ans: &mut Vec<String>, path: &mut [u8], digits: &[u8], map: &[&str]) {
            if i == digits.len() {
                ans.push(unsafe { String::from_utf8_unchecked(path.to_vec()) });
                return;
            }

            for ch in map[(digits[i] - b'0') as usize].bytes() {
                path[i] = ch;
                dfs(i + 1, ans, path, digits, map);
            }
        }

        let digits = digits.as_bytes();
        let mut ans = vec![];
        let mut path = vec![0; n];

        dfs(0, &mut ans, &mut path, digits, &Self::MAPPING);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_letter_combinations{
            Solution::letter_combinations;

            "2".to_string() => vec!["a","b","c"];
            "23".to_string() => vec!["ad","ae","af","bd","be","bf","cd","ce","cf"];
        }
    );
}
