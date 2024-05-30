# 2597. The Number of Beautiful Subsets
- [Submission](https://leetcode.com/submissions/detail/1265569250/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 313 ms | 2.2 MB |
```
impl Solution {
    fn merge(i: usize, nums: &Vec<i32>, mut s: Vec<i32>, k: i32, bs: &mut i32) {
        if i >= nums.len() { return; }
        Self::merge(i+1, nums, s.clone(), k, bs);
        if let Err(_) = s.binary_search(&(nums[i]-k)) {
            *bs += 1;
            s.push(nums[i]);
            Self::merge(i+1, nums, s, k, bs);
        }
    }
    pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut bs = 0;
        Self::merge(0, &nums, vec![], k, &mut bs);
        bs
    }
}
```
