# 21. Merge Two Sorted Lists
- [Submission](https://leetcode.com/submissions/detail/1020934893/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.1 MB |
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = match (list1, list2) {
            (None, None) => return None,
            (Some(node), None)|(None, Some(node)) => return Some(node),
            (Some(a), Some(b)) => (Some(a), Some(b)),
        };
        let mut list = vec![];
        while list1!=None && list2!=None {
            let bruh1 = list1.unwrap();
            let bruh2 = list2.unwrap();
            if bruh1.val < bruh2.val {
                list.push(bruh1.val);
                list1 = bruh1.next;
                list2 = Some(bruh2);
            } else {
                list.push(bruh2.val);
                list2 = bruh2.next;
                list1 = Some(bruh1);
            } 
        }
        while list1!=None {
            let bruh = list1.unwrap();
            list.push(bruh.val);
            list1 = bruh.next;
        }
        while list2!=None {
            let bruh = list2.unwrap();
            list.push(bruh.val);
            list2 = bruh.next;
        }
        let mut ll = None;
        list.reverse();
        for i in list {
            ll = Some(Box::new(ListNode{next: ll, val: i,}))
        }
        ll
    }
}
```
