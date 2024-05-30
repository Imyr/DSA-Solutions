# 786. K-th Smallest Prime Fraction
- [Submission](https://leetcode.com/submissions/detail/1255478773/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 139 ms | 7.8 MB |
```
#[derive(Ord, Eq, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32
}

impl Fraction {
    fn new(n: i32, d: i32) -> Self {
        Self {numerator: n, denominator: d}
    }
    fn to_vec(self: Self) -> Vec<i32> {
        vec![self.numerator, self.denominator]
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.numerator*other.denominator).cmp(&(other.numerator*self.denominator)))
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..arr.len() {
            for j in i+1..arr.len() {
                heap.push(Fraction::new(arr[j], arr[i]));
            }
        }
        for _ in 1..k { heap.pop(); }
        heap.pop().unwrap().to_vec().into_iter().rev().collect()
    }
}
```
