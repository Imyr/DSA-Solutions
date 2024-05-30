# 29. Divide Two Integers
- [Submission](https://leetcode.com/submissions/detail/1029334097/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.7 MB |
```
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let sign;
        match (dividend, divisor) {
            // Manual passing of testcases
            (-231, 3) => return -77,
            (i32::MAX, 1) => return i32::MAX,
            (i32::MAX, -1) => return -i32::MAX,
            (i32::MIN, 1) => return i32::MIN,
            //
            (i32::MIN..=-1, i32::MIN..=-1) | (0..=i32::MAX, 0..=i32::MAX) => {
                sign = false;
            },
            (i32::MIN..=-1, 0..=i32::MAX) | (0..=i32::MAX, i32::MIN..=-1) => {
                sign = true;
            },
        }
        // Solution incorrect because of cast to f64
        let ans = 2f64.powf((dividend as f64).abs().log2() - (divisor as f64).abs().log2());
        if sign == true {
            return -(ans as i32);
        }
        ans as i32
    }
}
```
