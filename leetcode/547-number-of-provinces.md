# 547. Number of Provinces
- [Submission](https://leetcode.com/submissions/detail/1247677461/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.4 MB |
```
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut touched = vec![false; is_connected.len()];
        let mut pr = 0;
        for i in 0..is_connected.len() {
            if touched[i] { continue; }
            let mut stack = vec![i];
            while let Some(j) = stack.pop() {
                if touched[j] { continue; }
                for k in is_connected[j].iter().enumerate().filter(|(_, c)| **c==1).map(|(x, _)| x) {
                    if touched[k] { continue; }
                    stack.push(k);
                }
                touched[j] = true;
            }
            pr += 1;
        }
        pr as _
    }
}
```
