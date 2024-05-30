# 354. Russian Doll Envelopes
- [Submission](https://leetcode.com/submissions/detail/1261721627/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 39 ms | 9 MB |
```
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        envelopes.sort_unstable_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let mut bucket = vec![];
        for i in 0..n {
            let idx = bucket.binary_search(&envelopes[i][1]).unwrap_or_else(|x| x);
            if idx == bucket.len() { bucket.push(envelopes[i][1]); }
            else { bucket[idx] = envelopes[i][1]; }
        }
        bucket.len() as i32
    }
}
```
