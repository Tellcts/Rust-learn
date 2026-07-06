//! 字母异位词分组

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

        for str in strs {
            let mut sort_str = str.as_bytes().to_vec();
            sort_str.sort_unstable();
            map.entry(sort_str).or_default().push(str);
        }

        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO:测试用例待编写
    #[test]
    fn test_group_anagrams() {}
}
