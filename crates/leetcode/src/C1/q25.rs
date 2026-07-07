//! K个一组反转链表

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        let (mut n, mut cur) = (0, &head);

        // 统计节点个数
        while let Some(node) = cur {
            n += 1;
            cur = &node.next;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut p0 = &mut dummy;

        while n >= k {
            // 将K个节点摘出来作为一组进行反转，然后拼接回主链表
            let mut group_head = p0.next.take();
            let mut group_tail = &mut group_head;

            // 寻找尾节点
            for _ in 0..k - 1 {
                group_tail = &mut group_tail.as_mut().unwrap().next;
            }

            let rest = group_tail.as_mut().unwrap().next.take();
            let (mut pre, mut cur) = (rest, group_head);
            // 进行反转
            for _ in 0..k {
                let nxt = cur.as_mut().unwrap().next.take();

                cur.as_mut().unwrap().next = pre.take();
                pre = cur;
                cur = nxt;
            }

            // `p0`是`group_head`的上一个节点，将反转后的链表拼接回去
            p0.next = pre;

            // 更新`p0`为当前组的尾节点
            for _ in 0..k {
                p0 = p0.next.as_mut().unwrap();
            }

            n -= k;
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_reverse_k_group{
            Solution::reverse_k_group;

            None, 4 => None;
            list_from_vec(vec![1,2,3,4,5]), 2 => list_from_vec(vec![2,1,4,3,5]);
            list_from_vec(vec![1,2,3,4,5]), 3 => list_from_vec(vec![3,2,1,4,5]);
        }
    );
}
