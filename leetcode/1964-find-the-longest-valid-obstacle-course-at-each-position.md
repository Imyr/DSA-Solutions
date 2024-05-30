# 1964. Find the Longest Valid Obstacle Course at Each Position
- [Submission](https://leetcode.com/submissions/detail/1263301977/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 70 ms | 3.5 MB |
```
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        obstacles.into_iter().scan(vec![], |mobs, x| {
            let idx = mobs.partition_point(|&y| y<=x);
            if idx == mobs.len() { mobs.push(x) } else { mobs[idx] = x; }
            Some(idx as i32 + 1)
        }).collect()
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1263280654/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 86 ms | 4.4 MB |
```
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut mobs = vec![];
        for (i, &x) in obstacles.iter().enumerate() {
            match mobs.binary_search(&x) {
                Err(idx) => {
                    if idx == mobs.len() { mobs.push(x) } else { mobs[idx] = x; }
                    res.push(mobs.partition_point(|&y| !(y > x)) as i32);
                },
                Ok(idx) => {
                    let idx = mobs.partition_point(|&y| !(y > x));
                    if idx == mobs.len() { mobs.push(x); }
                    else if mobs[idx] > x { mobs[idx] = x; }
                    res.push(mobs.partition_point(|&y| !(y > x)) as i32);
                },
            }
        }
        res
    }
}
```
