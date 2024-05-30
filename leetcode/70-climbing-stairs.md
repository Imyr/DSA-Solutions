# 70. Climbing Stairs
- [Submission](https://leetcode.com/submissions/detail/1248260991/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (3..=n as usize).fold(vec![0, 1, 2], |mut acc, x| {
            acc.push(acc[x-1]+acc[x-2]);
            acc
        })[n as usize] as i32
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1041504394/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
use std::convert::TryInto;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n: u128 = n.try_into().unwrap();
        let mut ways = 0;
        let (mut twos, mut ones) 
            = match n%2 {
                0 => (n/2, 0),
                1 => ((n-1)/2, 1),
                _ => panic!()
            };

        while ones<=n {
            ways += (ones+1..=twos+ones).product::<u128>()/(1..=twos).product::<u128>();
            twos-=1;
            ones+=2;
        }
        ways as i32
    }
}
```
