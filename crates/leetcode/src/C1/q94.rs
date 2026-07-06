//! 二叉树的中序遍历

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = node {
                let n = node.borrow();
                dfs(&n.left, ans);
                ans.push(n.val);
                dfs(&n.right, ans);
            }
        }

        let mut ans = vec![];
        dfs(&root, &mut ans);

        ans
    }
}

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_node {
        ($val:expr) => {
            Rc::new(RefCell::new(TreeNode::new($val)))
        };
    }

    /// text
    /// ```
    ///      1
    ///    /   \
    ///   2     3   =>   [4, 2, 1, 3]
    ///  /
    /// 4
    /// ```
    #[test]
    fn test_inorder_traversal() {
        let root = make_node!(1);
        root.borrow_mut().left = Some(make_node!(2));
        root.borrow_mut().right = Some(make_node!(3));
        root.borrow_mut().left.as_mut().unwrap().borrow_mut().left = Some(make_node!(4));

        assert_eq!(Solution::inorder_traversal(Some(root)), vec![4, 2, 1, 3]);
    }
}
