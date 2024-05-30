# 338. Counting Bits
- [Submission](https://leetcode.com/submissions/detail/1042418075/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.7 MB |
```
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|mut x| {
            let mut count: i32 = 0;
            while x!=0 {
                count += x&1;
                x>>=1;
            }
            count
        }).collect()
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1037881535/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.6 MB |
```
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|x| x.count_ones() as i32).collect()
    }
}
```
