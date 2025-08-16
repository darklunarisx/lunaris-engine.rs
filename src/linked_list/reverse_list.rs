//! Reverse Singly Linked list (Generic, Production-Grade)
//!
//! Reverses a singly linked list in-place.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::linked_list::reverse_list::*;
//! use lunaris_engine::linked_list::singly_linked_list::ListNode;
//! let mut head = Some(Box::new(ListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
//! let rev = reverse_list(head);
//! ```
use crate::linked_list::singly_linked_list::ListNode;

pub fn reverse_list<T: Clone>(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    prev
}
