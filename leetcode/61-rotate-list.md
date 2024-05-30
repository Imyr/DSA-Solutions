# 61. Rotate List
- [Submission](https://leetcode.com/submissions/detail/1272277257/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2 MB |
```
impl Solution {
    fn recurse(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            let next = node.next;
            if n == 1 {
                node.next = None; (Some(node), next)      
            } else {
                let mut nude = None;
                (node.next, nude) = Self::recurse(next, n-1);
                (Some(node), nude)
            }
            
        } else { (None, None) }
    }
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (mut n, mut iter) = (0, head.clone());
        while let Some(node) = iter {
            n += 1;
            iter = node.next;
        }
        if head.is_none() || k%n == 0 { return head; }
        let (mut head, mut nrev) = Self::recurse(head, n-(k%n));
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
        head
    }
}
```
