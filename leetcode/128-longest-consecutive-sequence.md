# 128. Longest Consecutive Sequence
- [Submission](https://leetcode.com/submissions/detail/1263107460/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 14 ms | 3.3 MB |
```
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map: std::collections::HashSet<_> = nums.into_iter().collect();
        let mut max = 0;
        for i in map.iter() {
            if !map.contains(&(i-1)) {
                let mut j = 0;
                while { j+=1; map.contains(&(i+j)) } {}
                max = max.max(j);
            }
        }
        max
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1263103086/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1011 ms | 3.3 MB |
```
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map = nums.iter().map(|&x| x).collect::<std::collections::HashSet<_>>();
        let mut max = 0;
        for i in nums {
            if !map.contains(&(i-1)) {
                let mut j = 0;
                while { j+=1; map.contains(&(i+j)) } {}
                max = max.max(j);
            }
        }
        max
    }
}
```
