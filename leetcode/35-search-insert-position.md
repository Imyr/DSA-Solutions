# 35. Search Insert Position
- [Submission](https://leetcode.com/submissions/detail/1112458479/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2 MB |
```
trait Search {
    fn custom_bsi(&self, target: &i32, l: usize, r: usize) -> usize ;
}

impl Search for Vec<i32> {
    fn custom_bsi(&self, target: &i32, mut l: usize, mut r: usize) -> usize {
        let mut m;
        while l <= r {
            m = l + (r - l) / 2;
            if target < &self[m] {
                if m > 0 { 
                    r = m - 1;
                } else {
                    return m;
                }
            } else if &self[m] < target {
                if m < self.len() {
                    l = m + 1;
                } else {
                    return m;
                }
            } else {
                return m;
            }
        }
        return l;
    }
} 

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.custom_bsi(&target, 0, nums.len()-1) as i32
    }
}
```
