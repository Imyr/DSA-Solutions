# 1464. Maximum Product of Two Elements in an Array
- [Submission](https://leetcode.com/submissions/detail/1117953535/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut i1, mut i2) = (0, 0);       
        for e in nums {
            if e >= i1 {
                i2 = i1;
                i1 = e;
            } else if e > i2 {
                i2 = e;
            }
        }
        (i1-1)*(i2-1)
    }
}
```
