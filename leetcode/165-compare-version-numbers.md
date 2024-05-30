# 165. Compare Version Numbers
- [Submission](https://leetcode.com/submissions/detail/1248090752/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1.split('.').map(|x| x.parse::<i32>().unwrap()).collect();
        let v2: Vec<i32> = version2.split('.').map(|x| x.parse::<i32>().unwrap()).collect();
        
        let mut i = 0;
        while i < v1.len() && i < v2.len() {
            if v1[i] < v2[i] { return -1 }
            else if v1[i] > v2[i] { return 1 }
            i += 1;
        }
        while i < v1.len() {
            if v1[i] > 0 { return 1 }
            i += 1;
        }
        while i < v2.len() {
            if v2[i] > 0 { return -1 }
            i += 1;
        }
        0
    }
}
```
