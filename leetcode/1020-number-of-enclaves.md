# 1020. Number of Enclaves
- [Submission](https://leetcode.com/submissions/detail/1261707030/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 21 ms | 6.4 MB |
```
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut can_walk_off = vec![vec![-1; n]; m];
        let mut stack = vec![];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 { stack.push((i, j)); }
            }
        }
        while let Some((a, b)) = stack.pop() {
            if can_walk_off[a][b] != -1 { continue; }
            let mut s1 = vec![];
            let mut s2 = vec![];
            s1.push((a, b));
            let mut out = false;
            while let Some((i, j)) = s1.pop() {
                if i < 0 || j < 0 || i >= m || j >= n || can_walk_off[i][j] == 1 { out = true; continue; }
                if grid[i][j] == 0 || visited[i][j] { continue; }   
                if can_walk_off[i][j] == 1 { out = true; continue }
                visited[i][j] = true;             
                s2.push((i, j));
                s1.push((i-1, j));
                s1.push((i, j-1));
                s1.push((i+1, j));
                s1.push((i, j+1));
            }
            for (i, j) in s2.into_iter() {
                can_walk_off[i][j] = if out { 1 } else { 0 };
            }
        }
        can_walk_off.into_iter().map(|x| x.into_iter().filter(|&x| x==0).count()).sum::<usize>() as i32
    }
}
```
