# 121. Best Time to Buy and Sell Stock
- [Submission](https://leetcode.com/submissions/detail/1250015633/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 6 ms | 2.9 MB |
```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut i, mut j, mut max) = (0, 1, 0);
        while j < prices.len() {
            if prices[j] > prices[i] { max = max.max(prices[j]-prices[i]); } 
            else { i = j; }
            j += 1; 
        }
        max
    }
}
```
