# 71. Simplify Path
- [Submission](https://leetcode.com/submissions/detail/1266846382/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.4 MB |
```
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut word_stack: Vec<String> = vec![];
        let mut char_stack = vec![];
        let mut dot = 0;
        for j in path.chars() {
            match j {
                '/' => {
                    match dot {
                        0 | 1 => {},
                        2 => if !word_stack.is_empty() { word_stack.pop(); },
                        _ => word_stack.push(".".repeat(dot as usize)),
                    }
                    dot = 0;
                    if !char_stack.is_empty() {
                        word_stack.push(char_stack.into_iter().collect());
                        char_stack = vec![];
                    }
                },
                '.' =>{
                    if !char_stack.is_empty() { char_stack.push('.'); }
                    else { dot += 1; }
                },
                _ => {
                    if dot != 0 { for i in 0..dot as usize { char_stack.push('.'); } }
                    dot = 0;
                    char_stack.push(j);
                },
            }
        }
        if dot > 2 { word_stack.push(".".repeat(dot as usize)); }
        else if dot == 2 { word_stack.pop(); }
        if !char_stack.is_empty() { word_stack.push(char_stack.into_iter().collect()); }
        "/".to_string() + &word_stack.join("/")
    }
}
```
