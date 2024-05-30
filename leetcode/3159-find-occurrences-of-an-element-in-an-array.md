# 3159. Find Occurrences of an Element in an Array
- [Submission](https://leetcode.com/submissions/detail/1267547426/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 53 ms | 3.9 MB |
```
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut indices = vec![];
        for (i, y) in nums.into_iter().enumerate() {
            if x == y { indices.push(i as i32); }
        }
        let mut res = vec![];
        for i in queries {
            if i-1 >= indices.len() as i32 { res.push(-1); }
            else { res.push(indices[(i-1) as usize]); }
        }
        res
    }
}
```
