//! 合并两个有序链表

use crate::ListNode;

/// [灵茶山艾府](https://leetcode.cn/problems/merge-two-sorted-lists/solutions/2373691/liang-chong-fang-fa-die-dai-di-gui-pytho-wf75/)
pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut guard = ListNode::new(0);
        let mut cur = &mut guard;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            let min = if node1.val < node2.val {
                &mut list1
            } else {
                &mut list2
            };

            cur.next = min.take();
            cur = cur.next.as_mut()?;
            *min = cur.next.take();
        }

        cur.next = list1.or(list2);

        guard.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_merge_two_lists{
            Solution::merge_two_lists;

            list_from_vec(vec![1,2,4]), list_from_vec(vec![1,3,4]) =>
            list_from_vec(vec![1,1,2,3,4,4]);

            list_from_vec(vec![]), list_from_vec(vec![0]) =>
            list_from_vec(vec![0]);

            list_from_vec(vec![]), list_from_vec(vec![]) =>
            None;
        }
    );
}
