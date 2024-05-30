# 26. Remove Duplicates from Sorted Array
- [Submission](https://leetcode.com/submissions/detail/1023899488/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.3 MB |
```
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        for i in (1..nums.len()).rev() {
            if nums[i] == nums[i-1] {
                nums.remove(i);
            }
        }
        nums.len() as i32
    }
}
```
