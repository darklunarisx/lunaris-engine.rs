//! Palindrome Linked list Check (Generic, Production-Grade)
//!
//! Checks if a singly linked list is a palindrome.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `Eq`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::linked_list::palindrome::*;
//! use lunaris_engine::linked_list::singly_linked_list::ListNode;
//! let mut head = Some(Box::new(ListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
//! head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
//! assert!(is_palindrome(&head));
//! ```
use crate::linked_list::singly_linked_list::ListNode;

pub fn is_palindrome<T: Clone + Eq>(head: &Option<Box<ListNode<T>>>) -> bool {
    let mut vals = Vec::new();
    let mut node = head.as_ref();
    while let Some(n) = node {
        vals.push(n.val.clone());
        node = n.next.as_ref();
    }
    let n = vals.len();
    for i in 0..n/2 {
        if vals[i] != vals[n-1-i] {
            return false;
        }
    }
    true
}
