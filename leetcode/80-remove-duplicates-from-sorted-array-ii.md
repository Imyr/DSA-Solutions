# 80. Remove Duplicates from Sorted Array II
- [Submission](https://leetcode.com/submissions/detail/1248253157/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.2 MB |
```
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        for i in (2..nums.len()).rev() {
            if nums[i] == nums[i-1] && nums[i-1] == nums[i-2] {
                nums.remove(i);
            }
        }
        nums.len() as i32

    }
}
```
