# 704. Binary Search
- [Submission](https://leetcode.com/submissions/detail/1112481443/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.2 MB |
```
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;
        let mut m;
        while l <= r {
            m = l + (r - l) / 2;
            if target < nums[m] {
                if m > 0 { 
                    r = m - 1;
                } else {
                    return -1;
                }
            } else if nums[m] < target {
                if m < nums.len() {
                    l = m + 1;
                } else {
                    return -1;
                }
            } else {
                return m as i32;
            }
        }
        return -1;
    }
}
```
