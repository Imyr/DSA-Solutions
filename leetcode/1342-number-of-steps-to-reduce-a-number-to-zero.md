# 1342. Number of Steps to Reduce a Number to Zero
- [Submission](https://leetcode.com/submissions/detail/1035222381/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut i = 0;
        while num!=0 {
            match num&1 {
                0 => num>>=1,
                1 => num&=!1,
                _ => panic!()
            }
            i+=1;
        }
        i
    }
}
```
