# 857. Minimum Cost to Hire K Workers
- [Submission](https://leetcode.com/submissions/detail/1255507939/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.3 MB |
```
use std::fmt;
use std::collections::BinaryHeap;

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

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}


impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut ratio_heap = BinaryHeap::new();
        for i in 0..quality.len() {
            ratio_heap.push(Fraction::new(quality[i], wage[i]));
        
        }

        let mut quality_heap = BinaryHeap::new();
        let mut total_quality = 0;
        let mut ratio = 0f64;
        for _ in 0..k as usize { 
            if let Some(Fraction {numerator: quality, denominator: wage}) = ratio_heap.pop() {
                quality_heap.push(quality);
                total_quality += quality;
                ratio = wage as f64 / quality as f64;
            }
        }

        let mut total_wage = total_quality as f64 * ratio;
        while let Some(Fraction {numerator: quality, denominator: wage}) = ratio_heap.pop() {
            quality_heap.push(quality);
            total_quality += quality;
            total_quality -= quality_heap.pop().unwrap();
            ratio = wage as f64 / quality as f64;
            total_wage = total_wage.min(total_quality as f64 * ratio);
        }
        total_wage
    }
}
```
