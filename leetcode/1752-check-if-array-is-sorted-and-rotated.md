# 1752. Check if Array Is Sorted and Rotated
- [Submission](https://leetcode.com/submissions/detail/1079461026/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len()==1 {
            return true
        }
        let mut flip = false;
        for i in 0..nums.len()-1 {
            if nums[i]<=nums[i+1]  {
                continue;
            } else {
                if flip {
                    return false
                } else {
                    flip = true;
                }
            }
        }
        if flip {
            if nums.first()>=nums.last() {
                true
            } else {
                false
            }
        } else {
            true
        }  
    }
}
```
