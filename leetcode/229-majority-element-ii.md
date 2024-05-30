# 229. Majority Element II
- [Submission](https://leetcode.com/submissions/detail/1067435263/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.4 MB |
```
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len()/3;
        let mut unique = Vec::new();
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            match counter.get_mut(&num) {
                Some(x) => {*x+=1},
                None => {counter.insert(num, 1);},
            }
        }
        for (number, repetitions) in counter {
            if repetitions > n as i32 {
                unique.push(number);
            } 
        }
        unique
    }
}
```
