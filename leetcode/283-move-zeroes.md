# 283. Move Zeroes
- [Submission](https://leetcode.com/submissions/detail/1067895627/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 12 ms | 2.3 MB |
```
impl Solution {
pub fn move_zeroes(nums: &mut Vec<i32>) {
    for i in (0..nums.len()-1).rev() {
        if nums[i]==0 {
            nums.remove(i);
            nums.push(0);
        }
    }
}
}
```
