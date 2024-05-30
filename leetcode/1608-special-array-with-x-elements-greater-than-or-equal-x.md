# 1608. Special Array With X Elements Greater Than or Equal X
- [Submission](https://leetcode.com/submissions/detail/1269134981/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for i in 0..nums.len() {
            let x = (nums.len()-i) as i32;
            if nums[i] >= x && (if i == 0 { 0 } else { nums[i-1] }) < x { return x; }
        }
        -1
    }
}
```
