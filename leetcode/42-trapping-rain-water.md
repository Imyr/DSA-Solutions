# 42. Trapping Rain Water
- [Submission](https://leetcode.com/submissions/detail/1254311401/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.3 MB |
```
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut prefix_max = vec![0; height.len()];
        for i in 1..height.len() { prefix_max[i] = height[i-1].max(prefix_max[i-1]) };
        let mut suffix_max = vec![0; height.len()];
        for i in (0..height.len()-1).rev() { suffix_max[i] = height[i+1].max(suffix_max[i+1]) }
        let mut harvest = 0;
        for i in 1..height.len()-1 {
            harvest += (prefix_max[i].min(suffix_max[i]) - height[i]).max(0);
        }
        harvest
    }
}
```
