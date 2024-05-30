# 1035. Uncrossed Lines
- [Submission](https://leetcode.com/submissions/detail/1265150948/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.1 MB |
```
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        *(1..=nums1.len()).fold(vec![0i32; nums2.len()+1], |mut acc1, i| {
            acc1[nums2.len()] = (1..=nums2.len()).fold(0i32, |mut acc2, j| {
                (acc1[j-1], acc2) = (acc2, if nums1[i-1] == nums2[j-1] { acc1[j-1]+1 } else { acc2.max(acc1[j]) }); acc2
            }); acc1
        }).last().unwrap()
    }
}
```
