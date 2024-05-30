# 231. Power of Two
- [Submission](https://leetcode.com/submissions/detail/1027889906/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2 MB |
```
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n>0 && i32::count_ones(n)==1
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1026026256/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n>0 && n&n-1==0 
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1025068227/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        match n {
            i32::MIN..=-1 => return false,
            0..=i32::MAX => {
                if i32::count_ones(n) == 1 {
                    return true;
                }
                return false;
            }
        }
    }
}
```
