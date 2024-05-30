# 714. Best Time to Buy and Sell Stock with Transaction Fee
- [Submission](https://leetcode.com/submissions/detail/1267836791/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 111 ms | 12.3 MB |
```
use std::collections::HashMap;
impl Solution {
    fn sell(i: usize, prices: &Vec<i32>, fee: i32, cache: &mut HashMap<(bool, usize), i32>) -> i32 {
        if i >= prices.len() { return 0; }
        if let Some(profit) = cache.get(&(false, i)) { *profit }
        else {
            let max = Self::sell(i+1, prices, fee, cache)
            .max(prices[i] - fee + Self::buy(i+1, prices, fee, cache));
            cache.insert((false, i), max);
            max
        }
    }
    fn buy(i: usize, prices: &Vec<i32>, fee: i32, cache: &mut HashMap<(bool, usize), i32>) -> i32 {
        if i >= prices.len() { return 0; }
        if let Some(profit) = cache.get(&(true, i)) { *profit }
        else {
            let max = Self::buy(i+1, prices, fee, cache)
            .max(-prices[i] + Self::sell(i+1, prices, fee, cache));
            cache.insert((true, i), max);
            max
        }
    }
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::buy(0, &prices, fee, &mut cache)
    }
}
```
