# 167. Two Sum II - Input Array Is Sorted
- [Submission](https://leetcode.com/submissions/detail/1256050192/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len()-1 {
            match numbers[i+1..].binary_search(&(target-numbers[i])) {
                Ok(idx) =>  { return vec![(i+1) as i32, (i+idx+2) as i32]; }
                Err(_) => { continue; },
            }
        }
        unreachable!()
    }
}
```
