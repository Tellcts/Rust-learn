//! 寻找两个正序数组的中位数

pub struct Solution;

impl Solution {
    /// 暴力解法(均匀分组)
    /// 时间复杂度：O((m+n)log(m+n))，来源于排序算法
    /// 空间复杂度：O(m+n)
    pub fn find_median_sorted_arrays_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = nums1;
        merged.extend(nums2);
        merged.sort_unstable();

        let n = merged.len();
        // 中位数索引
        let idx = (n - 1) / 2;

        if n % 2 == 1 {
            merged[idx] as f64
        } else {
            (merged[idx] + merged[idx + 1]) as f64 / 2.0
        }
    }

    /// 双指针
    /// 时间复杂度：O(m+n)，来源于插入操作
    /// 空间复杂度：O(1)
    pub fn find_median_sorted_arrays_2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let (m, n) = (nums1.len(), nums2.len());
        nums1.insert(0, i32::MIN);
        nums2.insert(0, i32::MIN);
        nums1.push(i32::MAX);
        nums2.push(i32::MAX);

        let mut i = 0;
        let mut j = (m + n).div_ceil(2);

        while nums1[i + 1] <= nums2[j] {
            i += 1;
            j -= 1;
        }

        let max1 = nums1[i].max(nums2[j]);
        let min2 = nums1[i + 1].min(nums2[j + 1]);

        if (m + n) % 2 == 1 {
            max1 as f64
        } else {
            (max1 + min2) as f64 / 2.0
        }
    }

    /// 二分法
    /// 时间复杂度：O(log(min(m,n)))
    /// 空间复杂度：O(1)
    pub fn find_median_sorted_arrays_3(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let (m, n) = (nums1.len(), nums2.len());
        let (mut left, mut right) = (-1, m as i32);

        while left + 1 < right {
            let i = (left + right) as usize / 2;
            let j = (m + n - 1) / 2 - i;

            if nums1[i] < nums2[j] {
                left = i as i32;
            } else {
                right = i as i32;
            }
        }

        let i = left;
        let j = (m + n + 1) as i32 / 2 - i - 2;
        let nums1_i = if i >= 0 { nums1[i as usize] } else { i32::MIN };
        let nums2_j = if j >= 0 { nums2[j as usize] } else { i32::MIN };
        let ai1 = if i + 1 < m as i32 {
            nums1[(i + 1) as usize]
        } else {
            i32::MAX
        };

        let bj1 = if j + 1 < n as i32 {
            nums2[(j + 1) as usize]
        } else {
            i32::MAX
        };

        let max1 = nums1_i.max(nums2_j);
        let min2 = ai1.min(bj1);

        if (m + n) % 2 > 0 {
            max1 as f64
        } else {
            (max1 + min2) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test!(
        test_find_median_sorted_arrays{
            Solution::find_median_sorted_arrays_1;
            Solution::find_median_sorted_arrays_2;
            Solution::find_median_sorted_arrays_3;

            vec![1, 3], vec![2] => 2.0;
            vec![1, 2], vec![3, 4] => 2.5;
            vec![1, 1], vec![1, 1] => 1.0;
        }
    );
}
