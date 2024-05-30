# 1544. Make The String Great
- [Submission](https://leetcode.com/submissions/detail/1224327547/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s = s.bytes().collect::<Vec<_>>();
        let mut i = s.len()-1;

        let mut l = usize::MAX;
        while l != s.len() {
            l = s.len();
            while i >= 1 && i < s.len() {
                if s[i-1].abs_diff(s[i]) == 32 { 
                    s.remove(i);
                    s.remove(i-1);
                    i -= 1;
                } 
                i -= 1;
            }
            i = s.len()-1;
        }
        
        s.into_iter().map(|x| x as char).collect::<String>()
    }
}
```
