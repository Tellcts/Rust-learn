//! 单词拆分

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Recursive Solution
    pub fn word_break_recursive(s: String, word_dict: Vec<String>) -> bool {
        fn dfs(
            i: usize,
            max_len: usize,
            s: &str,
            words: &HashSet<String>,
            memo: &mut Vec<i32>,
        ) -> bool {
            if i == 0 {
                return true;
            }
            if memo[i] != -1 {
                return memo[i] == 1;
            }

            for j in (i.saturating_sub(max_len)..i).rev() {
                if words.contains(&s[j..i]) && dfs(j, max_len, s, words, memo) {
                    memo[i] = 1;
                    return true;
                }
            }

            memo[i] = 0;
            false
        }

        let n = s.len();
        let mut memo = vec![-1; n + 1];
        let max_len = word_dict.iter().map(|s| s.len()).max().unwrap();
        let words = word_dict.into_iter().collect::<HashSet<_>>();
        dfs(n, max_len, &s, &words, &mut memo)
    }

    /// DP Solution
    pub fn word_break_dp(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let max_len = word_dict.iter().map(|s| s.len()).max().unwrap();
        let words = word_dict.into_iter().collect::<HashSet<_>>();

        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for j in (i.saturating_sub(max_len)..i).rev() {
                if dp[j] && words.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_word_break{
            Solution::word_break_dp;
            Solution::word_break_recursive;

            "leetcode".to_string(), vec!["leet".to_string(), "code".to_string()] => true;

            "applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()] => true;

            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ] => false;
        }
    );
}
