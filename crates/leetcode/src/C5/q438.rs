//! 找到字符串中所有字母异位词
pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (s_len, p_len) = (s.len(), p.len());

        if s_len < p_len {
            return vec![];
        }

        let mut ans = vec![];
        let (mut s_cnt, mut p_cnt) = (vec![0_u8; 26], vec![0_u8; 26]);
        for i in 0..p_len {
            s_cnt[(s[i] - b'a') as usize] += 1;
            p_cnt[(p[i] - b'a') as usize] += 1;
        }
        if s_cnt == p_cnt {
            ans.push(0);
        }

        for j in 0..(s_len - p_len) {
            s_cnt[(s[j] - b'a') as usize] -= 1;
            s_cnt[(s[j + p_len] - b'a') as usize] += 1;
            if s_cnt == p_cnt {
                ans.push((j + 1) as i32);
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
        test_find_anagrams{
            Solution::find_anagrams;

            "abab".to_string(), "ab".to_string() => vec![0, 1, 2];
            "cbaebabacd".to_string(), "abc".to_string() => vec![0, 6];
        }
    );
}
