# 188. Best Time to Buy and Sell Stock IV
- [Submission](https://leetcode.com/submissions/detail/1269663937/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 53 ms | 14.6 MB |
```
use std::collections::HashMap;
impl Solution {
    fn sell(i: usize, prices: &Vec<i32>, k: i32, cache: &mut HashMap<(bool, usize, i32), i32>) -> i32 {
        if i >= prices.len() { return 0; }
        if let Some(profit) = cache.get(&(false, i, k)) { *profit }
        else {
            let max = Self::sell(i+1, prices, k, cache)
            .max(prices[i] + Self::buy(i+1, prices, k-1, cache));
            cache.insert((false, i, k), max);
            max
        }
    }
    fn buy(i: usize, prices: &Vec<i32>, k: i32, cache: &mut HashMap<(bool, usize, i32), i32>) -> i32 {
        if i >= prices.len() || k <= 0 { return 0; }
        if let Some(profit) = cache.get(&(true, i, k)) { *profit }
        else {
            let max = Self::buy(i+1, prices, k, cache)
            .max(-prices[i] + Self::sell(i+1, prices, k, cache));
            cache.insert((true, i, k), max);
            max
        }
    }
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Self::buy(0, &prices, k, &mut cache)
    }
}
```
