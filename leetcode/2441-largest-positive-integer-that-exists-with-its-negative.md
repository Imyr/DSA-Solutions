# 2441. Largest Positive Integer That Exists With Its Negative
- [Submission](https://leetcode.com/submissions/detail/1247335548/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let (mut i, mut j) = (0, nums.len()-1);
        while i < j {
            if -nums[i] == nums[j] { return nums[j].abs() }
            
            if nums[i].abs() > nums[j].abs() { i += 1; } 
            else if nums[i].abs() < nums[j].abs() { j -= 1; }
            else { break; }
        }
        -1
    }
}
```
