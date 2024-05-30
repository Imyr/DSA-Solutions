# 509. Fibonacci Number
- [Submission](https://leetcode.com/submissions/detail/1079453876/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut one,  mut two) = (0, 1);
        for _ in 2..=n as usize+1 {
            two = one + two;
            one = two - one;
        }
        one
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1079446856/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut table = vec![0, 1];
        for idx in 2..=n as usize{
            table.push(table[idx-1]+table[idx-2]);
        }
        table[n as usize]
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1079435602/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 6 ms | 2 MB |
```
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => Solution::fib(n-1) + Solution::fib(n-2),
        } 
    }
}
```
