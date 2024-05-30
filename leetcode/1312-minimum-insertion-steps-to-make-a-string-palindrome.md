# 1312. Minimum Insertion Steps to Make a String Palindrome
- [Submission](https://leetcode.com/submissions/detail/1265860273/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 2 MB |
```
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let (s, n) = (s.as_bytes(), s.len());
        n as i32 - (1..=n).fold(vec![0i32; n+1], |mut acc1, i| {
            acc1[n] = (1..=n).fold(0i32, |mut acc2, j| {
                (acc1[j-1], acc2) = (acc2, 
                    if s[i-1] == s[n-j] { acc1[j-1]+1 } else { acc2.max(acc1[j]) 
                }); acc2
            }); acc1
        })[n]
    }
}
```
