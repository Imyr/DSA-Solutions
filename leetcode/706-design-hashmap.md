# 706. Design HashMap
- [Submission](https://leetcode.com/submissions/detail/1066593009/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 27 ms | 11.4 MB |
```
struct MyHashMap {
    map: Vec<i32>
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap{map: vec![-1; 1000001]}
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.map[key as usize] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        self.map[key as usize]
    }
    
    fn remove(&mut self, key: i32) {
        self.map[key as usize] = -1;
    }
}
```
