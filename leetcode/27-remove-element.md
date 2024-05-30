# 27. Remove Element
- [Submission](https://leetcode.com/submissions/detail/1023897536/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in (0..nums.len()).rev() {
            if nums[i] == val {
                nums.remove(i);
            }
        }
        nums.len() as i32
    }
}
```
