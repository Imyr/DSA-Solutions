# 82. Remove Duplicates from Sorted List II
- [Submission](https://leetcode.com/submissions/detail/1271362516/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn dedup(prev: i32, head: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            let (next, mut next_node) = Self::dedup(node.val, node.next);
            if node.val != prev && node.val != next { 
                node.next = next_node;
                (node.val, Some(node))
            } else { (node.val, next_node) }
        } else { (-101, None) }
    }
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::dedup(-101, head).1
    }
}
```
