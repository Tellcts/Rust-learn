//! 正则表达式匹配（hard）

/// [灵茶山艾府](https://leetcode.cn/problems/regular-expression-matching/solutions/902845/go-yi-xing-by-endlesscheng-1vf3/)
pub struct Solution;

impl Solution {
    pub fn is_match_recursive(s: &str, p: &str) -> bool {
        fn dfs(s: &[u8], p: &[u8], memo: &mut [Vec<Option<bool>>], i: usize, j: usize) -> bool {
            if j == p.len() {
                return i == s.len();
            }

            if let Some(res) = memo[i][j] {
                return res;
            }

            let is_match = i < s.len() && (p[j] == b'.' || p[j] == s[i]);
            let res = if j + 1 < p.len() && p[j + 1] == b'*' {
                dfs(s, p, memo, i, j + 2) || (is_match && dfs(s, p, memo, i + 1, j))
            } else {
                is_match && dfs(s, p, memo, i + 1, j + 1)
            };

            memo[i][j] = Some(res);

            res
        }

        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        let mut memo = vec![vec![None; m + 1]; n + 1];

        dfs(s, p, &mut memo, 0, 0)
    }

    pub fn is_match_iterative(s: &str, p: &str) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        let mut dp = vec![vec![false; m + 1]; n + 1];

        dp[n][m] = true;

        for i in (0..=n).rev() {
            for j in (0..m).rev() {
                if p[j] == b'*' {
                    continue;
                }

                let is_match = i < n && (p[j] == b'.' || p[j] == s[i]);

                if j + 1 < m && p[j + 1] == b'*' {
                    dp[i][j] = dp[i][j + 2] || (is_match && dp[i + 1][j]);
                } else {
                    dp[i][j] = is_match && dp[i + 1][j + 1];
                }
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_is_match{
            Solution::is_match_recursive;
            Solution::is_match_iterative;

            "aa", "a" => false;
            "aa", "a*" => true;
            "ab", ".*" => true;
            "aab", "c*a*b" => true;
            "mississippi", "mis*is*p*." => false;
        }
    );
}
