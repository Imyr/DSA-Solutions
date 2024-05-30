# 1442. Count Triplets That Can Form Two Arrays of Equal XOR
- [Submission](https://leetcode.com/submissions/detail/1272288947/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 17 ms | 2.2 MB |
```
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        (0..n).fold(0, |mut acc, i| {
            (i+1..n).fold(0, |mut a, j| {
                a ^= arr[j-1];
                (j..n).fold(0, |mut b, k| {
                    b ^= arr[k]; if a == b { acc += 1; } b
                }); a
            }); acc
        })
    }
}
```
