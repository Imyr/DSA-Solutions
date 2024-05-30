# 224. Basic Calculator
- [Submission](https://leetcode.com/submissions/detail/1268521510/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.4 MB |
```
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut stack, mut num, mut res, mut sign) = (vec![], 0, 0, 1);
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(res);
                    stack.push(sign); //sign of parenthesis result
                    (res, sign) = (0, 1);
                },
                ')' => {
                    res += sign * num;
                    sign = stack.pop().unwrap();
                    res *= sign;
                    res += stack.pop().unwrap();
                    (num, sign) = (0, 1);
                },
                '+' => {
                    res += sign * num;
                    (num, sign) = (0, 1);
                },
                '-' => {
                    res += sign * num;
                    (num, sign) = (0, -1);
                },
                ' ' => continue, 
                _ => { num = num*10 + c.to_digit(10).unwrap() as i32; },
            }
        }
        res + (sign * num)    
    }
}

```
