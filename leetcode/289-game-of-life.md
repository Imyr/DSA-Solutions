# 289. Game of Life
- [Submission](https://leetcode.com/submissions/detail/1261075719/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (m,  n) = (board.len(), board[0].len());
        let mut next_tick = board.clone();
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                if i > 0 { sum += board[i-1][j]; }
                if j > 0 { sum += board[i][j-1]; }
                if i < m-1 { sum += board[i+1][j]; }
                if j < n-1 { sum += board[i][j+1]; }
                if i > 0 && j > 0 { sum += board[i-1][j-1]; }
                if i < m-1 && j > 0 { sum += board[i+1][j-1]; }
                if i > 0 && j < n-1 { sum += board[i-1][j+1]; }
                if i < m-1 && j < n-1 { sum += board[i+1][j+1]; }
                
                if sum < 2 || sum > 3 { next_tick[i][j] = 0; }
                else if sum == 3 { next_tick[i][j] = 1; }
            }
        } 
        *board = next_tick;
    }
}
```
