# 36. Valid Sudoku
- [Submission](https://leetcode.com/submissions/detail/1033448036/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut checker: HashSet<char> = HashSet::new();
        
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j]!='.' && !checker.insert(board[i][j]) {
                    return false;
                };
            }
            checker.drain();
        }

        for i in 0..9 {
            for j in 0..9 {
                if board[j][i]!='.' && !checker.insert(board[j][i]) {
                    return false;
                };
            }
            checker.drain();
        }

        for i in 0..3 {
            for j in 0..3 {
                for x in i*3..(i+1)*3 {
                    for y in j*3..(j+1)*3 {
                        if board[x][y]!='.' && !checker.insert(board[x][y]) {
                            return false;
                        };
                    }
                }
                checker.drain();
            }
        }
        true
    }
}
```
