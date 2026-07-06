use std::{cmp::Ordering, iter::FromIterator, ops::Not};

/// AVL-Tree Node Definition
#[derive(Debug)]
struct AVLNode<T: Ord> {
    value: T,
    // Height of the node
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLNode<T> {
    /// Get a reference to a child node
    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// Get a mutable reference to a child node
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// Get the height of a child node
    fn height(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |node| node.height)
    }

    /// Calculate the balance factor of a node
    ///
    /// balance_factor = right - left
    fn balance_factor(&self) -> i8 {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));

        if left < right {
            (right - left) as i8
        } else {
            -((left - right) as i8)
        }
    }

    /// Update the height of a node
    fn update_height(&mut self) {
        self.height = 1 + self.height(Side::Left).max(self.height(Side::Right))
    }

    /// Rotate a node
    fn rotate(&mut self, side: Side) {
        let mut subtree = self.child_mut(!side).take().unwrap();
        *self.child_mut(!side) = subtree.child_mut(side).take();
        self.update_height();
        // Swap root and child nodes in memory
        std::mem::swap(self, subtree.as_mut());
        // Set old root (subtree) as child of new root (self)
        *self.child_mut(side) = Some(subtree);
        self.update_height();
    }

    /// Rebalance a node
    fn rebalance(&mut self) {
        self.update_height();
        let side = match self.balance_factor() {
            -2 => Side::Left,
            2 => Side::Right,
            _ => return,
        };
        let subtree = self.child_mut(side).as_mut().unwrap();
        // Left-Right and Right-Left require rotation of heavy subtree
        if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
            subtree.rotate(side);
        }
        // Rotate in opposite direction of heavy side
        self.rotate(!side);
    }
}

/// AVL-Tree Definition
#[derive(Debug)]
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    len: usize,
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for AVLTree<T> {
    /// Create a new AVL-Tree from an iterator
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AVLTree::new();

        for value in iter {
            tree.insert(value);
        }

        tree
    }
}

impl<T: Ord> AVLTree<T> {
    /// Create a new empty AVL-Tree
    pub fn new() -> Self {
        AVLTree { root: None, len: 0 }
    }

    /// Get the number of elements in the AVL-Tree
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the AVL-Tree is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the AVL-Tree contains a value
    pub fn contains(&self, value: &T) -> bool {
        let mut cur = &self.root;

        while let Some(node) = cur {
            cur = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            };
        }

        false
    }

    /// Insert a value into the AVL-Tree
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = helper::insert(&mut self.root, value);

        if inserted {
            self.len += 1;
        }

        inserted
    }

    /// Remove a value from the AVL-Tree
    pub fn remove(&mut self, value: &T) -> bool {
        let removed = helper::remove(&mut self.root, value);

        if removed {
            self.len -= 1;
        }

        removed
    }

    /// Get the Iterator of value over the AVL-Tree
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            node_iter: self.node_iter(),
        }
    }

    /// Get the Iterator of AVL-Tree Node over the AVL-Tree
    fn node_iter(&self) -> NodeIter<'_, T> {
        let cap = self.root.as_ref().map_or(0, |node| node.height);
        let mut node_iter = NodeIter {
            stack: Vec::with_capacity(cap),
        };

        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node);
            child = &node.left;
        }

        node_iter
    }
}

/// AVL-Tree Node Iterator
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                let mut child = &node.right;
                while let Some(subtree) = child {
                    self.stack.push(subtree);
                    child = &subtree.left;
                }

                Some(node)
            }
        }
    }
}

/// AVL-Tree value Iterator
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_iter.next() {
            None => None,
            Some(node) => Some(&node.value),
        }
    }
}

/// Side Definition
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

/// Helper functions
pub(super) mod helper {
    use super::{AVLNode, Ordering};

    /// Insert a value into a tree
    pub(super) fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
        if let Some(node) = tree {
            let inserted = match value.cmp(&node.value) {
                Ordering::Equal => false,
                Ordering::Less => insert(&mut node.left, value),
                Ordering::Greater => insert(&mut node.right, value),
            };

            if inserted {
                node.rebalance();
            }

            inserted
        } else {
            *tree = Some(Box::new(AVLNode {
                value,
                height: 1,
                left: None,
                right: None,
            }));

            true
        }
    }

    /// Remove a value from a tree
    pub(super) fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
        match tree {
            None => false,
            Some(node) => {
                let removed = match value.cmp(&node.value) {
                    Ordering::Less => remove(&mut node.left, value),
                    Ordering::Greater => remove(&mut node.right, value),
                    Ordering::Equal => {
                        *tree = match (node.left.take(), node.right.take()) {
                            (None, None) => None,
                            (Some(b), None) | (None, Some(b)) => Some(b),
                            (Some(left), Some(right)) => Some(merge(left, right)),
                        };
                        return true;
                    }
                };
                if removed {
                    node.rebalance();
                }

                removed
            }
        }
    }

    /// Merge two trees
    fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut right = Some(right);
        let mut root = take_min(&mut right).unwrap();

        root.left = Some(left);
        root.right = right;
        root.rebalance();

        root
    }

    /// Take the minimum node from a tree
    fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
        match tree.take() {
            None => None,
            Some(mut node) => {
                if let Some(smaller) = take_min(&mut node.left) {
                    node.rebalance();
                    *tree = Some(node);
                    Some(smaller)
                } else {
                    *tree = node.right.take();
                    Some(node)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// Returns `true` if all nodes in the tree are balanced.
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: AVLTree<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn balanced() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
        }
    }
}
