# 123. Best Time to Buy and Sell Stock III
- [Submission](https://leetcode.com/submissions/detail/1268779334/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 3.5 MB |
```
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let (mut buy, mut profit) = (100001, -1);
        let mut first = vec![0; n];
        for i in 0..n {
            buy = buy.min(prices[i]);
            profit = profit.max(prices[i]-buy);
            first[i] = profit;
        }
        (buy, profit) = (-1, -1);
        let mut second = vec![0; n];
        for j in (0..n).rev() {
            buy = buy.max(prices[j]);
            profit = profit.max(buy - prices[j]);
            second[j] = profit;
        }
        let mut max_profit = 0;
        for i in 0..n-1 {
            max_profit = max_profit.max(first[i]+second[i+1])
        }
        max_profit.max(first[n-1])
    }
}

```
