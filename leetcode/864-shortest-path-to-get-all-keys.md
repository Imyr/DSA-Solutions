# 864. Shortest Path to Get All Keys
- [Submission](https://leetcode.com/submissions/detail/1260797462/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 89 ms | 8.4 MB |
```
impl Solution {
    pub fn shortest_path_all_keys(mut grid: Vec<String>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let grid = grid.into_iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();
        let (mut keys, mut start) = (0, (0, 0));
        for i in 0..m { for j in 0..n {
            match grid[i][j] {
                'a'..='z' => keys += 1,
                '@' => start = (i, j),
                _ => continue,
        } } }
        let keys = vec![false; keys];
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, start, keys));
        while let Some((depth, (i, j), mut keys)) = queue.pop_front() {
            if i < 0 || j < 0 || i > m-1 || j > n-1 || !visited.insert((keys.clone(), i, j)) { continue; }
            visited.insert((keys.clone(), i, j));
            match grid[i][j] {
                '#' => { continue; },
                'a'..='z' => {
                    keys[grid[i][j] as usize - 'a' as usize] = true;
                    if keys.iter().filter(|&&x| x).count() == keys.len() { return depth; }
                },
                'A'..='Z' => { if !keys[grid[i][j] as usize - 'A' as usize] { continue; } },
                _ => {},
            }
            queue.push_back((depth+1, (i+1, j), keys.clone()));
            queue.push_back((depth+1, (i, j+1), keys.clone()));
            queue.push_back((depth+1, (i-1, j), keys.clone()));
            queue.push_back((depth+1, (i, j-1), keys.clone()));
        }
        -1
    }
}
```
