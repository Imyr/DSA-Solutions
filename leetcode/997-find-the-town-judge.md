# 997. Find the Town Judge
- [Submission](https://leetcode.com/submissions/detail/1249437084/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 24 ms | 6.4 MB |
```
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let graph = trust.into_iter().fold(vec![vec![0; n as usize]; n as usize], |mut acc, i| {
            acc[i[1] as usize - 1][i[0] as usize - 1] = 1;
            acc
        });
        
        if let Some((idx, _)) = graph.iter().enumerate().find(
            |(idx, v)| v[*idx]==0 
            && v.iter().sum::<i32>()==n-1 
            && graph.iter().map(|x| x[*idx]).sum::<i32>()==0) { idx as i32 + 1 } else { -1 }
    }
}
```
