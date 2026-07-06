//! 有效的括号

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if !s.len().is_multiple_of(2) {
            return false;
        }

        let mapping = [(b')', b'('), (b']', b'['), (b'}', b'{')]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();

        let mut stack = vec![];

        for ch in s.bytes() {
            if !mapping.contains_key(&ch) {
                stack.push(ch);
            } else if stack.is_empty() || stack.pop().unwrap() != *mapping.get(&ch).unwrap() {
                return false;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_is_valid{
            Solution::is_valid;

            "()".to_string() => true;
            "()[]{}".to_string() => true;
            "){}[]".to_string() => false;
        }
    );
}
