# 122. Best Time to Buy and Sell Stock II
- [Submission](https://leetcode.com/submissions/detail/1250219069/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut i, mut j, mut max) = (0, 1, 0);
        while j < prices.len() {
            if prices[j] > prices[i] { max += prices[j]-prices[i]; } 
            i = j;
            j += 1; 
        }
        max
    }
}
```
