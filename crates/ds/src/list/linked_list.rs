use std::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    ptr::NonNull,
};

struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "-> {} {}", self.value, unsafe { node.as_ref() }),
            None => write!(f, "-> {}", self.value),
        }
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Node<T>>,
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.delete_head().is_some() {}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));

        node.next = self.head;
        node.prev = None;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.tail = node_ptr,
            Some(mut head_node) => unsafe { head_node.as_mut().prev = node_ptr },
        }

        self.head = node_ptr;
        self.len += 1;
    }

    pub fn insert_at_tail(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));

        node.prev = self.tail;
        node.next = None;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node_ptr,
            Some(mut tail_node) => unsafe { tail_node.as_mut().next = node_ptr },
        }

        self.tail = node_ptr;
        self.len += 1;
    }

    pub fn insert_at_ith(&mut self, index: usize, value: T) {
        if index > self.len {
            panic!("Index out of bounds")
        }

        if index == 0 || self.head.is_none() {
            self.insert_at_head(value);
            return;
        }

        if index == self.len {
            self.insert_at_tail(value);
            return;
        }

        let mut ith_node = self.head.unwrap();
        for _ in 0..index {
            unsafe {
                match ith_node.as_ref().next {
                    None => panic!("Index out of bounds"),
                    Some(next_node) => ith_node = next_node,
                }
            }
        }

        let mut node = Box::new(Node::new(value));
        unsafe {
            node.prev = ith_node.as_ref().prev;
            node.next = Some(ith_node);

            if let Some(mut prev_node) = ith_node.as_ref().prev {
                let node_ptr = NonNull::new(Box::into_raw(node));

                prev_node.as_mut().next = node_ptr;
                ith_node.as_mut().prev = node_ptr;
                self.len += 1;
            }
        }
    }

    pub fn delete_head(&mut self) -> Option<T> {
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());

            match old_head.next {
                None => self.tail = None,
                Some(mut next_node) => next_node.as_mut().prev = None,
            }

            self.head = old_head.next;
            self.len -= 1;

            old_head.value
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());

            match old_tail.prev {
                None => self.head = None,
                Some(mut prev_node) => prev_node.as_mut().next = None,
            }

            self.tail = old_tail.prev;
            self.len -= 1;

            old_tail.value
        })
    }

    pub fn delete_ith(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            return self.delete_head();
        }

        if index == self.len - 1 {
            return self.delete_tail();
        }

        let mut ith_node = self.head.unwrap();
        for _ in 0..index {
            unsafe {
                match ith_node.as_ref().next {
                    None => panic!("Index out of bounds"),
                    Some(next_node) => ith_node = next_node,
                }
            }
        }

        unsafe {
            let deleted = Box::from_raw(ith_node.as_ptr());

            if let Some(mut prev_node) = deleted.prev {
                prev_node.as_mut().next = deleted.next;
            }

            if let Some(mut next_node) = deleted.next {
                next_node.as_mut().prev = deleted.prev;
            }

            self.len -= 1;
            Some(deleted.value)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        Self::get_ith_node(self.head, index).map(|node_ptr| unsafe { &node_ptr.as_ref().value })
    }

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: usize) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(node_ptr) => match index {
                0 => Some(node_ptr),
                _ => Self::get_ith_node(unsafe { node_ptr.as_ref().next }, index - 1),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::LinkedList;

    #[test]
    fn insert_at_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_tail(1);
        list.insert_at_tail(second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }
    #[test]
    fn insert_at_head_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_head(1);
        list.insert_at_head(second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_tail() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 0);
        list.insert_at_ith(1, second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_head() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(0, second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_middle() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        let third_value = 3;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(1, second_value);
        list.insert_at_ith(1, third_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }

        match list.get(2) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_and_delete_at_ith_in_the_middle() {
        // Insert and delete in the middle of the list to ensure pointers are updated correctly
        let mut list = LinkedList::<i32>::new();
        let first_value = 0;
        let second_value = 1;
        let third_value = 2;
        let fourth_value = 3;

        list.insert_at_ith(0, first_value);
        list.insert_at_ith(1, fourth_value);
        list.insert_at_ith(1, third_value);
        list.insert_at_ith(1, second_value);

        list.delete_ith(2);
        list.insert_at_ith(2, third_value);

        for (i, expected) in [
            (0, first_value),
            (1, second_value),
            (2, third_value),
            (3, fourth_value),
        ] {
            match list.get(i) {
                Some(val) => assert_eq!(*val, expected),
                None => panic!("Expected to find {expected} at index {i}"),
            }
        }
    }

    #[test]
    fn insert_at_ith_and_delete_ith_work_over_many_iterations() {
        let mut list = LinkedList::<i32>::new();
        for i in 0..100 {
            list.insert_at_ith(i, i.try_into().unwrap());
        }

        // Pop even numbers to 50
        for i in 0..50 {
            println!("list.len {}", list.len);
            if i % 2 == 0 {
                list.delete_ith(i);
            }
        }

        assert_eq!(list.len, 75);

        // Insert even numbers back
        for i in 0..50 {
            if i % 2 == 0 {
                list.insert_at_ith(i, i.try_into().unwrap());
            }
        }

        assert_eq!(list.len, 100);

        // Ensure numbers were adderd back and we're able to traverse nodes
        if let Some(val) = list.get(78) {
            assert_eq!(*val, 78);
        } else {
            panic!("Expected to find 78 at index 78");
        }
    }

    #[test]
    fn delete_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_tail() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, first_value),
            None => panic!("Expected to find {first_value} at index 0"),
        }
    }

    #[test]
    fn delete_head_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at head"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn delete_ith_can_delete_at_tail() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        assert_eq!(list.len, 1);
    }

    #[test]
    fn delete_ith_can_delete_at_head() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at tail"),
        }

        assert_eq!(list.len, 1);
    }

    #[test]
    fn delete_ith_can_delete_in_middle() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        let third_value = 3;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        list.insert_at_tail(third_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        println!("Linked List is {list}");
        assert_eq!(3, list.len);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        list_str.insert_at_tail("C".to_string());
        println!("Linked List is {list_str}");
        assert_eq!(3, list_str.len);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        println!("Linked List is {list}");
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        println!("Linked List is {list_str}");
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn delete_ith_panics_if_index_equals_len() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        // len is 2, so index 2 is out of bounds
        list.delete_ith(2);
    }
}
