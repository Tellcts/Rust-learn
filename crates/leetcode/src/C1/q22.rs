//! 括号生成

/// [灵茶山艾府](https://leetcode.cn/problems/generate-parentheses/solutions/2071015/hui-su-bu-hui-xie-tao-lu-zai-ci-pythonja-wcdw/)
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: usize) -> Vec<String> {
        fn dfs(left: usize, right: usize, n: usize, path: &mut [u8], ans: &mut Vec<String>) {
            if right == n {
                ans.push(unsafe { String::from_utf8_unchecked(path.to_vec()) });
                return;
            }

            if left < n {
                path[left + right] = b'(';
                dfs(left + 1, right, n, path, ans);
            }

            if right < left {
                path[left + right] = b')';
                dfs(left, right + 1, n, path, ans);
            }
        }

        let mut ans = vec![];
        let mut path = vec![0; 2 * n];

        dfs(0, 0, n, &mut path, &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_generate_parenthesis{
            Solution::generate_parenthesis;

            1 => vec!["()"];
            2 => vec!["(())","()()"];
            3 => vec!["((()))","(()())","(())()","()(())","()()()"];
        }
    );
}
