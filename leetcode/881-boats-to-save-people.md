# 881. Boats to Save People
- [Submission](https://leetcode.com/submissions/detail/1248812174/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 9 ms | 2.5 MB |
```
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut boats = 0;
        let (mut i, mut j) = (0, people.len()-1);
        while i < j {
            if people[i]+people[j] > limit {
                boats += 1;
                j -= 1;
            } else {
                boats += 1;
                i += 1;
                j -= 1;
            }
        }
        if i == j { boats += 1 };
        boats
    }
}
```
