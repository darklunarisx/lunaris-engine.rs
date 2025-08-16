//! Examples for all linked_list
use lunaris_engine::linked_list::singly_linked_list::{ListNode, traverse};
use lunaris_engine::linked_list::insert_delete::{insert_at, delete_at};
use lunaris_engine::linked_list::doubly_linked_list::{DListNode, reverse_doubly};
use lunaris_engine::linked_list::reverse_list::reverse_list;
use lunaris_engine::linked_list::detect_cycle::has_cycle;
use lunaris_engine::linked_list::merge_sorted::merge_sorted;
use lunaris_engine::linked_list::remove_nth_from_end::remove_nth_from_end;
use lunaris_engine::linked_list::palindrome::is_palindrome;
use lunaris_engine::linked_list::intersection::intersection;

fn main() {
    // Singly linked list: 1 -> 2
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    println!("Traverse: {:?}", traverse(&head));
    // Insert at position
    insert_at(&mut head, 1, 3);
    println!("After insert: {:?}", traverse(&head));
    // Delete at position
    delete_at(&mut head, 1);
    println!("After delete: {:?}", traverse(&head));
    // Reverse singly linked list
    let rev = reverse_list(head.clone());
    println!("Reversed: {:?}", traverse(&rev));
    // Detect cycle
    println!("Has cycle: {}", has_cycle(&head));
    // Merge two sorted lists
    let l1 = Some(Box::new(ListNode::new(1)));
    let l2 = Some(Box::new(ListNode::new(2)));
    let merged = merge_sorted(l1, l2);
    println!("Merged: {:?}", traverse(&merged));
    // Remove nth from end
    let head2 = Some(Box::new(ListNode::new(1)));
    let head2 = remove_nth_from_end(head2, 1);
    println!("After remove nth: {:?}", traverse(&head2));
    // Palindrome check
    let mut pal = Some(Box::new(ListNode::new(1)));
    pal.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    pal.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    println!("Is palindrome: {}", is_palindrome(&pal));
    // Intersection
    let a = Some(Box::new(ListNode::new(1)));
    let b = Some(Box::new(ListNode::new(2)));
    println!("Intersection: {:?}", intersection(&a, &b));
    // Doubly linked list: 1 <-> 2
    let mut dhead = Some(Box::new(DListNode::new(1)));
    dhead.as_mut().unwrap().next = Some(Box::new(DListNode::new(2)));
    // Normally, prev pointers would be set, but for safety, reverse_doubly sets them to None
    let drev = reverse_doubly(&dhead);
    // Only print values
    let mut vals = Vec::new();
    let mut node = drev.as_ref();
    while let Some(n) = node {
        vals.push(n.val.clone());
        node = n.next.as_ref();
    }
    println!("Doubly reversed: {:?}", vals);
}
