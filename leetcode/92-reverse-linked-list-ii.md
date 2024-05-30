# 92. Reverse Linked List II
- [Submission](https://leetcode.com/submissions/detail/1270326286/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, mut left: i32, mut right: i32) -> Option<Box<ListNode>> {        
        let mut prev = None;
        while head.is_some() && left > 1 {
            let mut node = head.unwrap();
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            head = next;
            left -= 1; right -= 1;
        }
        let mut nrev = None; 
        while head.is_some() && right > 0 {
            let mut node = head.unwrap();
            let next = node.next;
            node.next = nrev;
            nrev = Some(node);
            head = next;
            right -= 1;
        }
        let mut rev = None;
        while let Some(mut node) = nrev {
            let next = node.next;
            node.next = rev;
            rev = Some(node);
            nrev = next;
        }
        while let Some(mut node) = rev {
            let next = node.next;
            node.next = head;
            head = Some(node);
            rev = next;
        }
        while let Some(mut node) = prev {
            let next = node.next;
            node.next = head;
            head = Some(node);
            prev = next;
        }
        head
    }
}
```
