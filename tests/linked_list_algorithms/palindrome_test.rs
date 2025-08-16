// Tests for Palindrome Linked list
use lunaris_engine::linked_list::singly_linked_list::*;
use lunaris_engine::linked_list::palindrome::is_palindrome;

#[test]
fn test_is_palindrome() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    assert!(is_palindrome(&head));
    let mut head2 = Some(Box::new(ListNode::new(1)));
    head2.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    assert!(!is_palindrome(&head2));
}
