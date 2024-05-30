# 413. Arithmetic Slices
- [Submission](https://leetcode.com/submissions/detail/1139407406/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut lengths = vec![];
        let counts = nums.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
        let mut count = 1;
        for i in 1..counts.len() {
            if counts[i-1] == counts[i] {
                count += 1;
            } else {
                if count > 1 {       
                    lengths.push(count-1);
                }
                count = 1;
            }
        }
        if count > 1 {
            lengths.push(count-1);
        }
        lengths.iter().map(|n| n*(n+1)/2).sum()
    }
}
```
