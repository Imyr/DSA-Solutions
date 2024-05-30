# 146. LRU Cache
- [Submission](https://leetcode.com/submissions/detail/1111472729/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 105 ms | 103.7 MB |
```
use std::collections::HashMap;

#[derive(Debug)]
struct CacheNode {
    key: i32,
    value: i32,
    prev_key: Option<i32>,
    next_key: Option<i32>,
}

#[derive(Debug)]
pub struct LRUCache {
    capacity: i32,
    size: i32,
    cache: HashMap<i32, CacheNode>,
    head_key: Option<i32>,
    tail_key: Option<i32>,
}

impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        LRUCache { 
            capacity: capacity,
            size: 0,
            cache: HashMap::with_capacity(capacity as usize), 
            head_key: None, 
            tail_key: None, 
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        match self.cache.get_mut(&key) {
            Some(node) => {
                if self.tail_key==Some(key){
                    node.value
                } else if self.head_key==Some(key) {
                    let value = node.value;
                    node.prev_key = self.tail_key;
                    self.head_key = self.cache.get(&key).unwrap().next_key;
                    self.cache.get_mut(&self.tail_key.unwrap()).unwrap().next_key = Some(key);
                    self.tail_key = Some(key);
                    value
                } else if self.head_key!=self.tail_key {
                    let value = node.value;
                    let current_prev_key = node.prev_key.unwrap();
                    let current_next_key = node.next_key.unwrap();
                    node.prev_key = self.tail_key;
                    self.cache.get_mut(&self.tail_key.unwrap()).unwrap().next_key = Some(key);
                    self.cache.get_mut(&current_prev_key).unwrap().next_key = Some(current_next_key);
                    self.cache.get_mut(&current_next_key).unwrap().prev_key = Some(current_prev_key);
                    self.tail_key = Some(key);
                    value
                } else {
                    node.value
                }
            },
            None => -1,
        }
    }
    
    fn delete(&mut self) {
        if self.head_key==self.tail_key {
            self.cache.remove(&self.head_key.unwrap()).unwrap();
            self.head_key = None;
            self.tail_key = None;
            return;
        } else {
            let head_key = self.head_key.unwrap();
            let next_head_key = self.cache.get(&head_key).unwrap().next_key.unwrap();
            self.cache.get_mut(&next_head_key).unwrap().prev_key = None;
            self.cache.remove(&head_key).unwrap();  
            self.head_key = Some(next_head_key);
        }
        
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.get_mut(&key).unwrap().value = value;
            self.get(key);
            return;
        }
        if self.size == self.capacity {
            self.delete()
        } else {
            self.size+=1
        }
        match (self.head_key, self.tail_key) {
            (None, None) => {
                self.cache.insert(key, CacheNode { key: key, value: value, prev_key: None, next_key: None });
                self.head_key = Some(key);
                self.tail_key = Some(key);
            },
            (Some(_), Some(tail)) => {
                self.cache.insert(key, CacheNode { key: key, value: value, prev_key: Some(tail), next_key: None });
                self.tail_key = Some(key);
                self.cache.get_mut(&tail).unwrap().next_key = Some(key);
            },
            _ => unreachable!()
        }
    }
}
```
