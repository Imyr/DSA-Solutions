# 1582. Special Positions in a Binary Matrix
- [Submission](https://leetcode.com/submissions/detail/1119085562/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.1 MB |
```
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut vec1 = vec![-1; mat.len()];
        for i in 0..mat.len() {
            let mut count = 0;
            let mut idx = 0;
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    count += 1;
                    idx = j;
                }
            }
            if count == 1 {
                vec1[i] = idx as i32;
            }
        }
        let mut vec2 = vec![-1; mat[0].len()];
        for i in 0..mat[0].len() {
            let mut count = 0;
            let mut idx = 0;
            for j in 0..mat.len() {
                if mat[j][i] == 1 {
                    count += 1;
                    idx = i;
                }
            }
            if count == 1 {
                vec2[i] = idx as i32;
            }
        }
        vec1.iter().filter(|&&x| x != -1 && vec2.contains(&x)).count() as i32
    }
}
```
