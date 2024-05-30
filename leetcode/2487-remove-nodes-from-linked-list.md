# 2487. Remove Nodes From Linked List
- [Submission](https://leetcode.com/submissions/detail/1251150281/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 71 ms | 9.9 MB |
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
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut rev_list, mut max) = (None, 0);
        while let Some(mut node) = head {
            head = node.next;
            node.next = rev_list;
            rev_list = Some(node);
        }
        while let Some(mut node) = rev_list {
            max = max.max(node.val);
            rev_list = node.next;
            if node.val >= max {
                node.next = head;
                head = Some(node);
            }
        }
        head
    }
}
```
