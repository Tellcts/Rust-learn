use std::cmp::Ordering;

/// Binary Search Tree (Bst) implementation in Rust.
pub struct Bst<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<Bst<T>>>,
    right: Option<Box<Bst<T>>>,
}

impl<T> Default for Bst<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Bst<T>
where
    T: Ord,
{
    /// Create a new empty BST.
    pub fn new() -> Self {
        Bst {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Find a value in the BST.Returns true if the target is found, false otherwise.
    pub fn search(&self, target: &T) -> bool {
        match &self.value {
            // empty tree
            None => false,
            // non-empty tree
            Some(key) => match target.cmp(key) {
                // target == root's key
                Ordering::Equal => true,
                // target < root's key
                Ordering::Less => {
                    if let Some(node) = &self.left {
                        node.search(target)
                    } else {
                        false
                    }
                }
                // target > root's key
                Ordering::Greater => {
                    if let Some(node) = &self.right {
                        node.search(target)
                    } else {
                        false
                    }
                }
            },
        }
    }

    /// Return a new iterator over the BST in-order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BstIter::new(self)
    }

    /// Insert a value into the correct position in this tree.
    pub fn insert(&mut self, value: T) {
        match &self.value {
            // empty tree
            None => self.value = Some(value),
            // non-empty tree
            Some(key) => {
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };

                match target_node {
                    None => {
                        let mut node = Bst::new();
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                    Some(node) => {
                        node.insert(value);
                    }
                }
            }
        }
    }

    /// Returns the minumum value in the BST.
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            None => self.value.as_ref(),
            Some(node) => node.minimum(),
        }
    }

    /// Returns the maximum value in the BST.
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            None => self.value.as_ref(),
            Some(node) => node.maximum(),
        }
    }

    /// Returns the largest value less than or equal to the value.
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            // empty tree
            None => None,
            // non-empty tree
            Some(key) => match value.cmp(key) {
                // value == root's key
                Ordering::Equal => Some(key),
                // value < root's key
                Ordering::Less => {
                    if let Some(node) = &self.left {
                        node.floor(value)
                    } else {
                        None
                    }
                }
                // value > root's key
                Ordering::Greater => match &self.right {
                    // right child is None
                    None => Some(key),
                    // right child is not None
                    Some(node) => {
                        let val = node.floor(value);
                        match val {
                            None => Some(key),
                            Some(_) => val,
                        }
                    }
                },
            },
        }
    }

    /// Returns the smallest value greater than or equal to the value.
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            // empty tree
            None => None,
            // non-empty tree
            Some(key) => match value.cmp(key) {
                // value == root's key
                Ordering::Equal => Some(key),
                // value > root's key
                Ordering::Greater => {
                    if let Some(node) = &self.right {
                        node.ceil(value)
                    } else {
                        None
                    }
                }
                // value < root's key
                Ordering::Less => match &self.left {
                    // left child is None
                    None => Some(key),
                    // left child is not None
                    Some(node) => {
                        let val = node.ceil(value);
                        match val {
                            None => Some(key),
                            Some(_) => val,
                        }
                    }
                },
            },
        }
    }
}

struct BstIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a Bst<T>>,
}

impl<T> BstIter<'_, T>
where
    T: Ord,
{
    pub fn new(tree: &Bst<T>) -> BstIter<'_, T> {
        let mut iter = BstIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BstIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                if let Some(right_node) = &node.right {
                    self.stack.push(right_node);
                    self.stack_push_left();
                }
                node.value.as_ref()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bst;

    fn prequel_memes_tree() -> Bst<&'static str> {
        let mut tree = Bst::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: Bst<i32> = Bst::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
