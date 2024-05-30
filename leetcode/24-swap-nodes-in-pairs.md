# 24. Swap Nodes in Pairs
- [Submission](https://leetcode.com/submissions/detail/1022711087/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None
        }
        let mut node1 = head.unwrap();
        if node1.next.is_none() {
            return Some(node1);
        }
        let mut node2 = node1.next.unwrap();
        node1.next = Solution::swap_pairs(node2.next);
        node2.next = Some(node1);
        Some(node2)
    }
}
```
