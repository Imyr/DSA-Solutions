# 1306. Jump Game III
- [Submission](https://leetcode.com/submissions/detail/1256820647/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 4 ms | 2.6 MB |
```
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start as usize);
        while let Some(i) = queue.pop_front() {
            if visited[i] { continue; }
            visited[i] = true;
            if arr[i] == 0 { return true; }
            let d = arr[i] as usize;
            if i >= d { queue.push_back(i-d); }
            if i + d < n { queue.push_back(i+d); }
        }
        false
    }
}
```
