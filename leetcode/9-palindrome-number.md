# 9. Palindrome Number
- [Submission](https://leetcode.com/submissions/detail/981690793/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 12 ms | 2 MB |
```
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut palindrome = 0;
        if x < 0 {
            return false;
        }
        let mut y = x;
        while y != 0 {
            palindrome = palindrome*10 + y%10;
            y/=10;
        }
        if palindrome == x {
            return true;
        } else {
            return false;
        }
    }
}
```
