# 1512. Number of Good Pairs
- [Submission](https://leetcode.com/submissions/detail/1065699128/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut occurences = vec![0; 101];
        for num in nums {
            occurences[num as usize] += 1;
        }
        let mut pairs = 0;
        for i in occurences {
            pairs += (0..i as i32).sum::<i32>();
        }
        pairs
    }
}
```
