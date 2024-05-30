# 300. Longest Increasing Subsequence
- [Submission](https://leetcode.com/submissions/detail/1263349349/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(vec![], |mut inc, x| {
            if let Err(idx) = inc.binary_search(&x) {
                if idx == inc.len() { inc.push(x); } 
                else { inc[idx] = x; }
            };
            inc
        }).len() as i32
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1137720929/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 66 ms | 2.5 MB |
```
impl Solution {
    fn llis(i: usize, seq: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if i >= seq.len() {
            0
        } else if cache[i] != 0 {
            cache[i]
        } else {
            let mut sub = vec![];
            for j in i+1..seq.len() {
                if seq[i] < seq[j] {
                    sub.push(1 + Solution::llis(j, seq, cache))
                }
            } 
            cache[i] = *sub.iter().max().unwrap_or(&1);
            cache[i]
        }
    }

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; nums.len()];
        for i in 0..nums.len() {
            Solution::llis(i, &nums, &mut cache);
        }
        *cache.iter().max().unwrap()
    }
}
```
