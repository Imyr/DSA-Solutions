# 1249. Minimum Remove to Make Valid Parentheses
- [Submission](https://leetcode.com/submissions/detail/1225080509/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn min_remove_to_make_valid(mut s: String) -> String {
        let mut remove = vec![];
        let mut left_b = vec![];
        for (idx, c) in s.chars().enumerate() {
            match c {
                '(' => left_b.push(idx),
                ')' => if left_b.pop().is_none() { remove.push(idx); },
                _ => {},
            }
        }
        remove.append(&mut left_b);
        remove.sort_unstable();
        s.chars().enumerate().filter(|(x, _)| remove.binary_search(x).is_err()).map(|(_, y)| y).collect()
    }
}
```
