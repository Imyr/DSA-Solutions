# 209. Minimum Size Subarray Sum
- [Submission](https://leetcode.com/submissions/detail/1257218048/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 9 ms | 3 MB |
```
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let prefix = nums.iter().map(|x| *x).fold(vec![], |mut acc, x| {
            acc.push(*acc.last().unwrap_or(&0)+x);
            acc
        });
        let mut min = nums.len()+1;
        for i in 0..nums.len() {
            match prefix[i+1..].binary_search(&(prefix[i]+target)) {
                Ok(j) => min = min.min(j+1),
                Err(j) => if i+j+1 != nums.len() { min = min.min(j+1) },
            } 
        }
        match prefix.binary_search(&target) { 
            Ok(i) => { min = min.min(i+1) },
            Err(i) => if i != nums.len() { min = min.min(i+1) },
        }
        if min == nums.len()+1 { 0 } else { min as i32 }
    }
}
```
