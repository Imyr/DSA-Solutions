# 1466. Reorder Routes to Make All Paths Lead to the City Zero
- [Submission](https://leetcode.com/submissions/detail/1251090534/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 61 ms | 10.9 MB |
```
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut reverse = 0;
        let mut visited = vec![false; n as usize];
        let connections = connections.into_iter().fold(std::collections::HashMap::new(), 
            |mut acc, x| { 
                acc.entry(x[0] as usize).or_insert(vec![]).push((x[1] as usize, true));
                acc.entry(x[1] as usize).or_insert(vec![]).push((x[0] as usize, false));
                acc
            }
        );
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        while let Some(city) = queue.pop_front() {
            visited[city] = true;
            if connections.get(&city).is_none() { continue; }
            for (x, b) in connections[&city].iter() {
                if visited[*x] { continue; }
                if !b { reverse += 1; }
                queue.push_back(*x);
            }
        }
        n - 1 - reverse
    }
}
```
