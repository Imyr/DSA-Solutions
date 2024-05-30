# 1700. Number of Students Unable to Eat Lunch
- [Submission](https://leetcode.com/submissions/detail/1226559607/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut sandwiches: std::collections::VecDeque<_> = sandwiches.into();
        let mut students = students.into_iter().fold(vec![0; 2], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });
        while !sandwiches.is_empty() {
            if students[*sandwiches.front().unwrap() as usize] == 0 { break; }
            students[sandwiches.pop_front().unwrap() as usize] -= 1;
        }
        students[0] + students[1]
    }
}
```
