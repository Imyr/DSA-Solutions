# 2073. Time Needed to Buy Tickets
- [Submission](https://leetcode.com/submissions/detail/1228013071/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut time = 0;
        let k = k as usize;
        for i in 0..=k { 
            tickets[i] -= 1;  
            time += 1;
        }
        tickets.iter().fold(vec![0; 101], 
        |mut acc, x| {
            acc[*x as usize] += 1;
            acc
        }).into_iter().enumerate().fold(0, |acc, (idx, x)| {
            acc + (x * if idx < tickets[k] as usize { idx as i32 } else { tickets[k] })
        })
        +
        time
    }
}
```
