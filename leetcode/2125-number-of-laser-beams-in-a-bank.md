# 2125. Number of Laser Beams in a Bank
- [Submission](https://leetcode.com/submissions/detail/1135735169/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 6 ms | 3 MB |
```
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter().map(|floor| floor.as_bytes().iter().filter(|x| **x == b'1').count() as i32)
        .filter(|y| *y != 0).collect::<Vec<i32>>()
        .windows(2).map(|z| z[0] * z[1]).sum::<i32>()
    }
}
```
