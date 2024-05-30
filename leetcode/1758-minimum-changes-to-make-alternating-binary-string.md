# 1758. Minimum Changes To Make Alternating Binary String
- [Submission](https://leetcode.com/submissions/detail/1127692347/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut c1, mut c2) = (0, 0);
        for x in 0..s.len() {
            match s[x] {
                '0' => {
                    if x%2==0 {
                        c1 += 1
                    } else {
                        c2 += 1
                    }
                },
                '1' => {
                    if x%2==0 {
                        c2 += 1
                    } else {
                        c1 += 1
                    }
                },
                _ => unreachable!()
            }    
        }
        std::cmp::min(c1, c2)       
    }
}
```
