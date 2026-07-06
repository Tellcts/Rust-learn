//! 合并K个升序链表

use crate::ListNode;
use std::{cmp::Reverse, collections::BinaryHeap};

/// [灵茶山艾府](https://leetcode.cn/problems/merge-k-sorted-lists/solutions/2384305/liang-chong-fang-fa-zui-xiao-dui-fen-zhi-zbzx/)
pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ans = None;
        let mut cur = &mut ans;
        let mut min_heap = BinaryHeap::new();

        for head in lists.into_iter().flatten() {
            min_heap.push(Reverse(head));
        }

        while let Some(mut node) = min_heap.pop() {
            if let Some(nxt_node) = node.0.next.take() {
                min_heap.push(Reverse(nxt_node));
            }

            cur = &mut cur.insert(node.0).next;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_merge_k_lists{
            Solution::merge_k_lists;

            vec![list_from_vec(vec![1,4,5]),list_from_vec(vec![1,3,4]),list_from_vec(vec![2,6])] =>
            list_from_vec(vec![1,1,2,3,4,4,5,6]);

            vec![] => None;

            vec![None] => None;
        }
    );
}
