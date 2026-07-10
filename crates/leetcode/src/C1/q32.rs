//! 最长有效括号

/// [灵茶山艾府](https://leetcode.cn/problems/longest-valid-parentheses/solutions/3843096/on-chai-zha-dan-tong-guo-li-zi-fa-ming-s-lb5s/)
pub struct Solution;

impl Solution {
    pub fn longest_valid_parenthesis_1(s: String) -> usize {
        let n = s.len();
        let mut is_valid = vec![false; n];
        let mut stack = vec![];

        for (idx, ch) in s.bytes().enumerate() {
            if ch == b'(' {
                stack.push(idx);
            } else if !stack.is_empty() {
                is_valid[idx] = true;
                is_valid[stack.pop().unwrap()] = true;
            }
        }

        let (mut ans, mut cnt) = (0, 0);

        for b in is_valid {
            if b {
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                cnt = 0;
            }
        }

        ans
    }

    pub fn longest_valid_parenthesis_2(s: String) -> usize {
        let mut stack = vec![-1];
        let mut ans = 0;

        for (idx, ch) in s.bytes().enumerate() {
            if ch == b'(' {
                stack.push(idx as i32);
            } else if stack.len() > 1 {
                stack.pop();
                ans = ans.max(idx as i32 - *stack.last().unwrap());
            } else {
                stack[0] = idx as i32;
            }
        }

        ans as _
    }

    pub fn longest_valid_parenthesis_3(s: String) -> usize {
        let ans1 = Self::solve(s.bytes(), b'(');
        let ans2 = Self::solve(s.bytes().rev(), b')');

        ans1.max(ans2)
    }

    fn solve(iter: impl Iterator<Item = u8>, left_ch: u8) -> usize {
        let mut ans = 0;
        let (mut left, mut right) = (0, 0);

        for ch in iter {
            if ch == left_ch {
                left += 1;
            } else {
                right += 1;
            }

            if left < right {
                (left, right) = (0, 0);
            } else if left == right {
                ans = ans.max(right * 2);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_longest_valid_parenthesis{
            Solution::longest_valid_parenthesis_1;
            Solution::longest_valid_parenthesis_2;
            Solution::longest_valid_parenthesis_3;

            String::new() => 0;
            String::from("())") => 2;
            String::from(")()())") => 4;
        }
    );
}
