//! 两两交换链表中的节点

use crate::ListNode;

/// [灵茶山艾府](https://leetcode.cn/problems/swap-nodes-in-pairs/solutions/2374872/tu-jie-die-dai-di-gui-yi-zhang-tu-miao-d-51ap/)
pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut node0 = &mut dummy;

        while let Some(node1) = node0.next.as_ref() {
            if node1.next.is_some() {
                let mut first = node0.next.take().unwrap();
                let mut second = first.next.take().unwrap();
                let third = second.next.take();

                node0.next = Some(second);
                node0.next.as_mut().unwrap().next = Some(first);
                node0.next.as_mut().unwrap().next.as_mut().unwrap().next = third;

                node0 = node0.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                break;
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_swap_pairs{
            Solution::swap_pairs;

            None => None;
            list_from_vec(vec![1]) => list_from_vec(vec![1]);
            list_from_vec(vec![1,2,3,4]) => list_from_vec(vec![2,1,4,3]);
            list_from_vec(vec![1,2,3,4,5]) => list_from_vec(vec![2,1,4,3,5]);
        }
    );
}
