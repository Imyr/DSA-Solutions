# 134. Gas Station
- [Submission](https://leetcode.com/submissions/detail/1253906641/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 3.3 MB |
```
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() { return -1 }
        let mut tank = 0;
        let mut pivot = 0;
        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            if tank < 0 { tank = 0; pivot = i + 1; }
        }
        pivot as i32
    }
}
```
