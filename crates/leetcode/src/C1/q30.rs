//! 串联所有单词的子串

use std::collections::HashMap;

/// [灵茶山艾府](https://leetcode.cn/problems/substring-with-concatenation-of-all-words/solutions/3691292/30-ci-ding-chang-hua-dong-chuang-kou-pyt-5vgx/)
pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<usize> {
        let word_len = words[0].len();
        let window_len = word_len * words.len();
        let mut target_cnt = HashMap::new();

        for w in &words {
            *target_cnt.entry(w.as_str()).or_insert(0) += 1;
        }

        let mut ans = vec![];

        for start in 0..word_len {
            let mut cnt = HashMap::new();
            let mut overload = 0;

            for right in (start + word_len..=s.len()).step_by(word_len) {
                let in_word = &s[right - word_len..right];
                let e = cnt.entry(in_word).or_insert(0);

                if e == target_cnt.get(in_word).unwrap_or(&0) {
                    overload += 1;
                }

                *e += 1;

                if right < window_len {
                    continue;
                }

                let left = right - window_len;

                if overload == 0 {
                    ans.push(left);
                }

                let out_word = &s[left..left + word_len];
                let e = cnt.entry(out_word).or_insert(0);
                *e -= 1;

                if e == target_cnt.get(out_word).unwrap_or(&0) {
                    overload -= 1;
                }
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
        test_find_substring{
            Solution::find_substring;

            String::from("barfoothefoobarman"),
            vec![String::from("foo"),String::from("bar")]
            => vec![0,9];

            String::from("wordgoodgoodgoodbestword"),
            vec![String::from("word"),String::from("good"),String::from("best"),String::from("word")]
            => vec![];

            String::from("barfoofoobarthefoobarman"),
            vec![String::from("bar"),String::from("foo"),String::from("the")]
            => vec![6,9,12];
        }
    );
}
