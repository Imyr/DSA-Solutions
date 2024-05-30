# 380. Insert Delete GetRandom O(1)
- [Submission](https://leetcode.com/submissions/detail/1147974152/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 56 ms | 30.3 MB |
```
use rand::Rng;
use std::collections::{HashMap, VecDeque};

struct RandomizedSet {
    set: HashMap<i32, bool>,
    vec: VecDeque<i32>
}   

impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet { set: HashMap::new(), vec: VecDeque::new() }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if let Some(_) = self.set.get(&val) {
            false
        } else {
            self.set.insert(val, true);
            self.vec.push_back(val);
            true
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(_) = self.set.remove(&val) {
            self.vec.remove(self.vec.iter().position(|&x| x==val).unwrap());
            true
        } else {
            false
        }
    }
    
    fn get_random(&mut self) -> i32 {
        let z = rand::thread_rng().gen_range(0, self.vec.len());
        self.vec[z]
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1147954422/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 135 ms | 29.9 MB |
```
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

struct RandomizedSet {
    set: HashMap<i32, bool>
}   

impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet { set: HashMap::new() }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if let Some(_) = self.set.get(&val) {
            false
        } else {
            self.set.insert(val, true);
            true
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(_) = self.set.remove(&val) {
            true
        } else {
            false
        }
    }
    
    fn get_random(&self) -> i32 {
        let z = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        +SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()%11
        +SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()%13
        +SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()%17
        +SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()%19;
        *self.set.keys().collect::<Vec<&i32>>()[
            (z % self.set.len() as u128) as usize
        ]
    }
}
```
