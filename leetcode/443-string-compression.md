# 443. String Compression
- [Submission](https://leetcode.com/submissions/detail/1130832379/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.5 MB |
```
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut new = String::new();
        let mut check = chars[0];
        let mut count = 1;
        for i in 1..chars.len() {
            if check == chars[i]{
                count += 1;
            } else {
                if count == 1 {
                    new = format!("{}{}", new, check);
                } else {
                    new = format!("{}{}{}", new, check, count);
                }
                check = chars[i];
                count = 1;
            }
        }
        if count == 1 {
            new = format!("{}{}", new, check);
        } else {
            new = format!("{}{}{}", new, check, count);
        }
        *chars = new.chars().collect::<Vec<char>>();
        new.len() as i32    
    }    
}
```
