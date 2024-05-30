# 1913. Maximum Product Difference Between Two Pairs
- [Submission](https://leetcode.com/submissions/detail/1122711623/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.4 MB |
```
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let (mut i1, mut i2) = (0, 0);      
        let (mut a1, mut a2) = (10001, 10001);       
        for e in nums {
            if e >= i1 {
                i2 = i1;
                i1 = e;
            } else if e > i2 {
                i2 = e;
            }
            if e <= a1 {
                a2 = a1;
                a1 = e;
            } else if e < a2 {
                a2 = e;
            }
        }
        i1*i2 - a1*a2
    }
}
```
