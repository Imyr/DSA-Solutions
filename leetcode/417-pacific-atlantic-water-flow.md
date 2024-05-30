# 417. Pacific Atlantic Water Flow
- [Submission](https://leetcode.com/submissions/detail/1264848769/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 6 ms | 2.3 MB |
```
impl Solution {
    fn dfs(prev: i32, i: usize, j: usize, heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) {
        if heights[i][j] < prev || visited[i][j] { return; }
        visited[i][j] = true;
        if i!=heights.len()-1 { Solution::dfs(heights[i][j], i+1, j, heights, visited) };
        if i!=0 { Solution::dfs(heights[i][j], i-1, j, heights, visited) };
        if j!=heights[0].len()-1 { Solution::dfs(heights[i][j], i, j+1, heights, visited) };
        if j!=0 { Solution::dfs(heights[i][j], i, j-1, heights, visited) };
    }
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pacific = vec![vec![false; heights[0].len()]; heights.len()];
        let mut atlantic = vec![vec![false; heights[0].len()]; heights.len()];
        for i in 0..heights.len() { 
            Solution::dfs(i32::MIN, i, 0, &heights, &mut pacific);
            Solution::dfs(i32::MIN, i, heights[0].len()-1, &heights, &mut atlantic);
        }
        for j in 0..heights[0].len() { 
            Solution::dfs(i32::MIN, 0, j, &heights, &mut pacific);
            Solution::dfs(i32::MIN, heights.len()-1, j, &heights, &mut atlantic);
        }
        let mut result = vec![];
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                if pacific[i][j] && atlantic[i][j] { result.push(vec![i as i32, j as i32]); }
            }
        }
        result
    }
}
```
