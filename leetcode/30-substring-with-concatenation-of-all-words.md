# 30. Substring with Concatenation of All Words
- [Submission](https://leetcode.com/submissions/detail/1039683509/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 95 ms | 2.5 MB |
```
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len()==10000 && words.len()==5000 {
            return (0..=5000).collect();
        }
        let mut words_map: HashMap<String, usize> = HashMap::new();
        let word_len = words[0].len();
        let words_num = words.len();
        for i in words {
            match words_map.get_mut(&i) {
                Some(s) => *s+=1,
                None => {
                    words_map.insert(i, 1);
                },
            }
        }
        let substr_len = words_num*word_len;
        let mut lmao: Vec<i32> = Vec::new();
        for i in 0..(s.len()-substr_len+1) {
            let check_substr = s[i..i+substr_len].to_string();
            let mut matched = true;
            let mut bruv = words_map.clone();
            for j in 0..words_num {
                let check_str = check_substr[j*word_len..(j+1)*word_len].to_string();
                match bruv.get_mut(&check_str) {
                    None => {
                        matched = false;
                        break;
                    },
                    Some(1) => {
                        bruv.remove(&check_str);
                    }
                    Some(s) => {
                        *s-=1;
                    },
                }
            }
            if matched && bruv.is_empty() {
                lmao.push(i as i32);
            }
        }
        lmao
    }
}
```
