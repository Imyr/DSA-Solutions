# 155. Min Stack
- [Submission](https://leetcode.com/submissions/detail/1267874440/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 6 ms | 5.9 MB |
```

struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {

    fn new() -> Self {
        MinStack{ stack: vec![] }
    }
    
    fn push(&mut self, val: i32) {
        if self.stack.is_empty() { self.stack.push((val, val)); }
        else { self.stack.push((val, val.min(self.stack.last().unwrap().1))) }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

```
