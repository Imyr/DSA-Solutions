# 19. Remove Nth Node From End of List
- [Submission](https://leetcode.com/submissions/detail/1019985340/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut iter = head.clone();
        while head!=None {
            head = head.unwrap().next;
            len+=1;
        }
        let mut values = vec![];
        for _ in 1..=len-n {
            let bruh = iter.unwrap();
            values.push(bruh.val);
            iter = bruh.next;
        }
        iter = match iter {
            Some(node) => node.next,
            None => None,
        };
        values.reverse();
        for value in values {
            iter = Some(Box::new(ListNode{val: value, next: iter}));
        }
        iter
    }
}
```
