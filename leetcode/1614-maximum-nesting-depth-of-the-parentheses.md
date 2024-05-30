# 1614. Maximum Nesting Depth of the Parentheses
- [Submission](https://leetcode.com/submissions/detail/1222838050/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.2 MB |
```
impl Solution {
    pub fn max_depth(s: String) -> i32 {   
        let mut max = 0;
        let mut size = 0;
        for c in s.chars() {
            match c {
                '(' => size+=1,
                ')' => size-=1,
                _ => {},
            }
            max = std::cmp::max(max, size)
        }  
        max
    }
}
```
