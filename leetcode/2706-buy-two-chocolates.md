# 2706. Buy Two Chocolates
- [Submission](https://leetcode.com/submissions/detail/1124313597/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (mut a1, mut a2) = (101, 101);       
        for e in prices {
            if e <= a1 {
                a2 = a1;
                a1 = e;
            } else if e < a2 {
                a2 = e;
            }
        }    
        if money-(a1+a2) < 0 {money} else {money-(a1+a2)}
    }
}
```
