# 1688. Count of Matches in Tournament
- [Submission](https://leetcode.com/submissions/detail/1112883779/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2 MB |
```
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        if n==1 {
            return 0
        }
        match n%2 {
            0 => {
                let matches_in_this_round = n/2;
                return matches_in_this_round + Solution::number_of_matches(matches_in_this_round) 
            },
            1 => {
                let matches_in_this_round = (n-1)/2;
                return matches_in_this_round + Solution::number_of_matches(matches_in_this_round+1) 

            },
            _ => unreachable!(),
        }
    }
}
```
