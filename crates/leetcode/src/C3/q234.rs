//! 回文链表

use crate::ListNode;

pub struct Solution;

impl Solution {
    /// HELPER FUNCTION: Find the middle node.
    fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut slow, mut fast) = (head, head);
        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        #[allow(mutable_transmutes)]
        let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
        slow.take()
    }

    /// HELPER FUNCTION: Reverse the linked list.
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    /// MAIN FUNCTION
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mid = Self::middle_node(&head);
        let mut head2 = Self::reverse_list(mid);

        while head.is_some() {
            if head.as_ref().unwrap().val != head2.as_ref().unwrap().val {
                return false;
            }

            head = head.unwrap().next;
            head2 = head2.unwrap().next;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list_from_vec, test};

    test!(
        test_is_palindrome{
            Solution::is_palindrome;

            list_from_vec(vec![1, 2]) => false;
            list_from_vec(vec![0, 0, 0]) => true;
            list_from_vec(vec![1, 2, 2, 1]) => true;
            list_from_vec(vec![1, 2, 3, 4, 3, 2, 1]) => true;
        }
    );
}
