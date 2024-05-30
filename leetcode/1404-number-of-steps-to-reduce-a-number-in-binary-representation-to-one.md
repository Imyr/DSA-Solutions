# 1404. Number of Steps to Reduce a Number in Binary Representation to One
- [Submission](https://leetcode.com/submissions/detail/1271253243/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let (mut steps, mut carry) = (0, false);
        for i in (1..n).rev() {
            match (s[i], carry) {
                (b'0', false) => { steps += 1; }, 
                (b'0', true) => { steps += 2; }, 
                (b'1', false) => { steps += 2; carry = true; },
                (b'1', true) => { steps += 1; }, 
                _ => unreachable!(),
            }
        }
        steps + if carry { 1 } else { 0 }
    }
}
```
