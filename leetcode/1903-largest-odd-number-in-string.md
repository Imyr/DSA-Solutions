# 1903. Largest Odd Number in String
- [Submission](https://leetcode.com/submissions/detail/1114177703/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.5 MB |
```
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        match num.chars().rev().position(|c| c.to_digit(10).unwrap()%2!=0) {
            Some(idx) => num[0..num.len()-idx].to_string(),
            None => String::new(),
        }
    }
}
```
