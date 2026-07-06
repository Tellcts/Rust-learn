//! 反转链表

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            let nxt = node.next;
            node.next = pre;
            pre = Some(node);
            cur = nxt;
        }

        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    // Rust
    // ```
    // [1 -> 2 -> 3 -> 4 -> 5]   =>   [5 -> 4 -> 3 -> 2 -> 1]
    // ```
    test!(
        test_reverse_list{
            Solution::reverse_list;

            list_from_vec(vec![1, 2, 3, 4, 5]) => list_from_vec(vec![5, 4, 3, 2, 1]);
            list_from_vec(vec![0]) => list_from_vec(vec![0]);
            list_from_vec(vec![]) => list_from_vec(vec![]);
        }
    );
}
