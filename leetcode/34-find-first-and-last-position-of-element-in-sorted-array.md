# 34. Find First and Last Position of Element in Sorted Array
- [Submission](https://leetcode.com/submissions/detail/1112440779/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_index = -1;
    let mut r = nums.len();
    while left_index==-1 {
        match nums[0..r].binary_search(&target) {
            Err(_) => return vec![-1, -1],
            Ok(idx) => {
                let idx = idx as usize;
                if idx==0 || nums[idx-1]!=target {
                    left_index = idx as i32;
                } else {
                    r = idx;
                }
            },
        } 
    }

    let mut right_index = -1;
    let mut l = 0;
    while right_index==-1 {
        match nums[l..nums.len()].binary_search(&target) {
            Err(_) => unreachable!(),
            Ok(idx) => {
                if l+idx+1==nums.len() || nums[l+idx+1]!=target {
                    right_index = (l+idx) as i32;
                } else {
                    l = l+idx+1;
                }
            },
        } 
    }

    vec![left_index, right_index]
}
}
```
