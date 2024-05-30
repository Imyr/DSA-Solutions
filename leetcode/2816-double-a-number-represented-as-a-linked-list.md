# 2816. Double a Number Represented as a Linked List
- [Submission](https://leetcode.com/submissions/detail/1251659398/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 38 ms | 2.9 MB |
```
impl Solution {
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rev_list = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = rev_list;
            rev_list = Some(node);
        }
        rev_list
    }
    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head = Solution::reverse(head);
        let mut doub = None;
        let mut carry = 0;
        while let Some(mut node) = head {
            let v = node.val * 2 + carry;
            node.val = v % 10;
            carry = v / 10;
            head = node.next;
            node.next = doub;
            doub = Some(node);
        }
        if carry == 1 { 
            let mut node = ListNode::new(1);
            node.next = doub;
            Some(Box::new(node))
        } else { doub }
    }
}
```
