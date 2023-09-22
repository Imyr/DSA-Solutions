use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut checker1: HashSet<char> = HashSet::new();
    let mut checker2: HashSet<char> = HashSet::new();
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j]!='.' && !checker1.insert(board[i][j]) {
                return false;
            };
            if board[j][i]!='.' && !checker2.insert(board[j][i]) {
                return false;
            };
        }
        checker1.drain();
        checker2.drain();
    }
    for i in 0..3 {
        for j in 0..3 {
            for x in i*3..(i+1)*3 {
                for y in j*3..(j+1)*3 {
                    if board[x][y]!='.' && !checker1.insert(board[x][y]) {
                        return false;
                    };
                }
            }
            checker2.drain();
        }
    }
    true
}