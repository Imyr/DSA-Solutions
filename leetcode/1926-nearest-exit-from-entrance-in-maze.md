# 1926. Nearest Exit from Entrance in Maze
- [Submission](https://leetcode.com/submissions/detail/1253932770/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 26 ms | 2.9 MB |
```
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, entrance[0] as usize, entrance[1] as usize));
        while let Some((depth, i, j)) = queue.pop_front() {
            if i < 0 || j < 0 || i >= maze.len() || j >= maze[i].len() || maze[i][j] == '+' || visited[i][j] { continue; }
            if depth != 0 && maze[i][j] == '.' && (i == 0 || j == 0 || i == maze.len()-1 || j == maze[0].len()-1) { return depth; }
            visited[i][j] = true;
            queue.push_back((depth+1, i+1, j));
            queue.push_back((depth+1, i, j+1));
            queue.push_back((depth+1, i-1, j));
            queue.push_back((depth+1, i, j-1));
        }
        -1
    }
}
```
