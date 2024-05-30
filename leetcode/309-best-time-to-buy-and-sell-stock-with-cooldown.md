# 309. Best Time to Buy and Sell Stock with Cooldown
- [Submission](https://leetcode.com/submissions/detail/1266553453/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 141 ms | 2.5 MB |
```
use std::collections::HashMap;
impl Solution {
    fn sell(bought_at: usize, i: usize, prices: &Vec<i32>, cache: &mut HashMap<usize, i32>) -> i32 {
        if i >= prices.len() { return 0; }
        Self::sell(bought_at, i+1, prices, cache)
        .max(prices[i] - prices[bought_at] + 
            if let Some(&max) = cache.get(&(i+2)) { max }
            else { let max = Self::buy(i+2, prices, cache); cache.insert(i+2, max); max }
        )
    }
    fn buy(i: usize, prices: &Vec<i32>, cache: &mut HashMap<usize, i32>) -> i32 {
        if i >= prices.len() { return 0; }
        Self::sell(i, i+1, prices, cache)
        .max(
            if let Some(&max) = cache.get(&(i+1)) { max }
            else { let max = Self::buy(i+1, prices, cache); cache.insert(i+2, max); max }
        )
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Self::buy(0, &prices, &mut cache)
    }
}
```
