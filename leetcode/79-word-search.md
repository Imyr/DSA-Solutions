# 79. Word Search
- [Submission](https://leetcode.com/submissions/detail/1222183607/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 370 ms | 2.1 MB |
```
impl Solution {
    pub fn check(board: &Vec<Vec<char>>, i: usize, j: usize, word: &Vec<char>, l: usize, mut crossed: Vec<Vec<bool>>) -> bool {
        if l >= word.len() {
            return true;
        } else if i >= board.len() || j >= board[0].len() || crossed[i][j] {
            return false;
        } else if board[i][j] == word[l] {
            crossed[i][j] = true;
            Solution::check(board, i+1, j, word, l+1, crossed.clone())
            ||
            Solution::check(board, i, j+1, word, l+1, crossed.clone())
            ||
            Solution::check(board, i-1, j, word, l+1, crossed.clone())
            ||
            Solution::check(board, i, j-1, word, l+1, crossed.clone())
        } else {
            return false;
        }
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut brd = board.clone().iter().fold(vec![], |mut acc, x| {
            acc.append(&mut x.clone());
            acc
        });
        brd.sort_unstable();
        for i in word.chars() {
            if brd.binary_search(&i).is_err() {
                println!("hello bro {}", i);
                return false;

            }
        }
        let mut crossed;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                crossed = vec![vec![false; board[0].len()]; board.len()];
                if Solution::check(&board, i, j, &word.chars().collect(), 0, crossed) { return true; }
            }
        }
        false
    }
}
```
