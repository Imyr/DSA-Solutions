# 225. Implement Stack using Queues
- [Submission](https://leetcode.com/submissions/detail/1079377035/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2 MB |
```
use std::collections::VecDeque;

struct MyStack {
    peeped: bool,
    queue: VecDeque<i32>,
}

impl MyStack {

    fn new() -> Self {
        MyStack { peeped: false, queue: VecDeque::new() }
    }
    
    fn push(&mut self, x: i32) {
        if self.empty() || !self.peeped {
            self.queue.push_back(x);
        } else {
            let temp = self.queue.pop_front().unwrap();
            self.queue.push_back(temp);
            self.queue.push_back(x);
            self.peeped = false;
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.empty() {
            return -1
        }
        if self.peeped {
            self.peeped = false;
            self.queue.pop_front().unwrap()
        } else {
            for _ in 1..self.queue.len() {
                let temp = self.queue.pop_front().unwrap();
                self.queue.push_back(temp);
            }
            self.queue.pop_front().unwrap()
        }
        
    }
    
    fn top(&mut self) -> i32 {
        if self.empty() {
            return -1
        }
        if self.peeped {
            *self.queue.front().unwrap()
        } else {
            for _ in 1..self.queue.len() {
                let temp = self.queue.pop_front().unwrap();
                self.queue.push_back(temp);
            }
            self.peeped = true;
            *self.queue.front().unwrap()
        }
        
    }
    
    fn empty(&self) -> bool {
        self.queue.len()==0
    }
}
```
