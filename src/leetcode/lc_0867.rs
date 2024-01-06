pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transpose = vec![vec![]; matrix[0].len()];
    for row in matrix {
        let mut idx = 0;
        while idx < row.len() {
            transpose[idx].push(row[idx]);
            idx += 1;
        }
    }
    transpose
}