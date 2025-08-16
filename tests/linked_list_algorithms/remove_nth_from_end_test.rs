// Tests for Remove Nth Node From End
use lunaris_engine::linked_list::singly_linked_list::*;
use lunaris_engine::linked_list::remove_nth_from_end::remove_nth_from_end;

#[test]
fn test_remove_nth_from_end() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    let head = remove_nth_from_end(head, 2);
    assert_eq!(traverse(&head), vec![1, 3]);
}
