# 540. Single Element in a Sorted Array
- [Submission](https://leetcode.com/submissions/detail/1112921358/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 3.1 MB |
```
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;
        let mut m;
        while l <= r {
            m = l + (r - l) / 2;
            if m>0 && nums[m-1]==nums[m] {
                if (m-1)%2==0 {
                    l = m + 1;
                } else {
                    r = m - 2;
                }
            } else if m<nums.len()-1 && nums[m]==nums[m+1] {
                if (m)%2==0 {
                    l = m + 2;
                } else {
                    r = m - 1;
                }
            } else {
                return nums[m]
            }
        }
        unreachable!()
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1111669727/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 3.1 MB |
```
impl Solution {
    pub fn single_non_duplicate(mut nums: Vec<i32>) -> i32 {
        let mut nums = nums.iter();
        let first = nums.next().unwrap();
        nums.fold(*first, |acc, x| acc^x)
    }
}
```
