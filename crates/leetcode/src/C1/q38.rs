//! 外观数列

pub struct Solution;

impl Solution {
    pub fn count_and_say(n: usize) -> String {
        let mut s = String::from("1");

        for _ in 0..n - 1 {
            let mut count_num: Vec<(i32, char)> = Vec::new();

            for ch in s.chars() {
                if let Some(last) = count_num.last_mut() {
                    if last.1 == ch {
                        last.0 += 1;
                    } else {
                        count_num.push((1, ch));
                    }
                } else {
                    count_num.push((1, ch));
                }
            }

            s = count_num
                .iter()
                .map(|(count, num)| format!("{}{}", count, num))
                .collect();
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_count_and_say{
            Solution::count_and_say;

            1 => String::from("1");
            4 => String::from("1211");
        }
    );
}
