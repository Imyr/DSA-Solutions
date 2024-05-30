# 673. Number of Longest Increasing Subsequence
- [Submission](https://leetcode.com/submissions/detail/1259089971/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.3 MB |
```
impl Solution {
    pub fn find_number_of_lis(mut nums: Vec<i32>) -> i32 {
        let (mut len, mut cnt) = (0, 0);
        let mut dp = vec![(0, 0); nums.len()];

        for i in (0..nums.len()).rev() {
            let (mut ml, mut mc) = (1, 1);
            for j in i+1..nums.len() {
                if nums[i] < nums[j] {
                    let (mut l, mut c) = dp[j];
                    if l + 1 > ml { ml = l+1; mc = c; }
                    else if l + 1 == ml { mc += c; }
                }
            }
            if len < ml { len = ml; cnt = mc; }
            else if len == ml { cnt += mc; }
            dp[i] = (ml, mc);
        }
        cnt
    }
}
```
