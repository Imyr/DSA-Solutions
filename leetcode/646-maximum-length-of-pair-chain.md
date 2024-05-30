# 646. Maximum Length of Pair Chain
- [Submission](https://leetcode.com/submissions/detail/1259632276/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 40 ms | 2.2 MB |
```
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.len();
        pairs.sort_unstable_by(|a, b| if a[1]==b[1] { a[0].cmp(&b[0]) } else { a[1].cmp(&b[1]) });
        let mut dp = vec![1; n];
        for i in (0..n).rev() {
            for j in (i+1..n).rev() {
                if pairs[i][1] < pairs[j][0] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }
}
```
