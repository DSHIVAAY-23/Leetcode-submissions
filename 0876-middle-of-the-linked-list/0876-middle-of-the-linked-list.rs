// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Use references to traverse the list
        let mut slow = &head;
        let mut fast = &head;

        // Move 'fast' two steps and 'slow' one step at a time
        while let Some(f_node) = fast {
            if let Some(next_f_node) = &f_node.next {
                fast = &next_f_node.next; // Move 'fast' two steps
                slow = &slow.as_ref()?.next; // Move 'slow' one step
            } else {
                break; // If fast can't move two steps, stop
            }
        }

        // Return the middle node (where 'slow' points to)
        slow.clone()
    }
}