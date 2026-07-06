//! 删除链表的倒数第N个结点

use crate::ListNode;

/// [灵茶山艾府](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/solutions/2004057/ru-he-shan-chu-jie-dian-liu-fen-zhong-ga-xpfs/)
pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let guard = ListNode { val: 0, next: head };
        let (mut left, mut right) = (&guard, &guard);

        for _ in 0..n {
            right = right.next.as_ref()?;
        }

        while let Some(ref node) = right.next {
            left = left.next.as_ref()?;
            right = node;
        }

        #[allow(mutable_transmutes)]
        let left: &mut ListNode = unsafe { std::mem::transmute(left) };

        left.next = left.next.take()?.next;

        guard.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_remove_nth_from_end{
            Solution::remove_nth_from_end;

            list_from_vec(vec![1]), 1 => None;
            list_from_vec(vec![1,2]), 1 => Some(Box::new(ListNode::new(1)));
            list_from_vec(vec![1,2,3,4,5]), 2 => list_from_vec(vec![1,2,3,5]);
        }
    );
}
