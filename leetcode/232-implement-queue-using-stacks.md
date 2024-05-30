# 232. Implement Queue using Stacks
- [Submission](https://leetcode.com/submissions/detail/1079402910/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
struct MyQueue {
    stack_push: Vec<i32>,
    stack_pop: Vec<i32>
}

impl MyQueue {

    fn new() -> Self {
        MyQueue {stack_push: Vec::new(), stack_pop: Vec::new()}
    }
    
    fn push(&mut self, x: i32) {
        self.stack_push.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.empty() {
            return -1;
        }
        if self.stack_pop.is_empty() {
            while !self.stack_push.is_empty() {
                self.stack_pop.push(self.stack_push.pop().unwrap());
            }
        }
        self.stack_pop.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.empty() {
            return -1;
        }
        if self.stack_pop.is_empty() {
            while !self.stack_push.is_empty() {
                self.stack_pop.push(self.stack_push.pop().unwrap());
            }
        }
        *self.stack_pop.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.stack_push.is_empty() && self.stack_pop.is_empty()
    }
}
```
