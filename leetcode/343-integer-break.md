# 343. Integer Break
- [Submission](https://leetcode.com/submissions/detail/1068500656/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n==2 {
            return 1;
        }
        if n==3 {
            return 2;
        }
        let mut max = 0;
        let mut i = 2;
        while n!=i {
            let mut temp = n;
            let mut prod = 1;
            while temp>=2*i {
                temp -= i;
                prod *= i;
            }
            if i*(temp-i) > temp {
                prod *= i*(temp-i);
            } else {
                prod *= temp;
            }
            if max <= prod {
                max = prod;
            } else {
                break;
            }
            i+=1;
        }
        max
    }
}
```
