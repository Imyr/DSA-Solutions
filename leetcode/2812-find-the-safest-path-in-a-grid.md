# 2812. Find the Safest Path in a Grid
- [Submission](https://leetcode.com/submissions/detail/1259022175/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 237 ms | 18.8 MB |
```
impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = std::collections::VecDeque::new();
        for i in 0..n { for j in 0..n { if grid[i][j] == 1 { 
            grid[i][j] = -1;
            queue.push_back((1, i-1, j)); 
            queue.push_back((1, i+1, j)); 
            queue.push_back((1, i, j-1)); 
            queue.push_back((1, i, j+1)); 
        } } }
        let mut visited = vec![vec![false; n]; n];
        while let Some((depth, i, j)) = queue.pop_front() {
            if i < 0 || j < 0 || i >= n || j >= n || grid[i][j] == -1 || visited[i][j] { continue; }
            visited[i][j] = true;
            grid[i][j] = depth;
            queue.push_back((depth+1, i-1, j)); 
            queue.push_back((depth+1, i+1, j)); 
            queue.push_back((depth+1, i, j-1)); 
            queue.push_back((depth+1, i, j+1)); 
        }
        println!("{:?}", grid);
        let mut heap = std::collections::BinaryHeap::new();
        let mut visited = vec![vec![false; n]; n];
        heap.push((grid[0][0], 0, 0));
        while let Some((mut min, i, j)) = heap.pop() {
            if i < 0 || j < 0 || i >= n || j >= n || grid[i][j] == -1 || visited[i][j] { continue; }
            visited[i][j] = true;
            min = min.min(grid[i][j]);
            if i == n-1 && j == n-1 { return min; }
            heap.push((min, i-1, j)); 
            heap.push((min, i+1, j)); 
            heap.push((min, i, j-1)); 
            heap.push((min, i, j+1)); 
        }
        0
    }
}
```
