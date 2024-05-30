# 344. Reverse String
- [Submission](https://leetcode.com/submissions/detail/1064150732/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 5.4 MB |
```
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len()-1;
        while i < j {
            s.swap(i, j);
            i+=1;
            j-=1;
        }  
    }
}
```
