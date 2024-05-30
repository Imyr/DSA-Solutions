# 17. Letter Combinations of a Phone Number
- [Submission](https://leetcode.com/submissions/detail/1019640780/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
use std::collections::HashMap;

impl Solution {
    fn merge_sc(v1: Vec<String>, v2: Vec<String>) -> Vec<String>{
        let mut new = Vec::new();
        for i in &v1 {
            for j in &v2 {
                new.push(format!("{}{}", i, j));
            }
        }
        new
}
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        let mut map: HashMap<char, Vec<String>> = HashMap::new();
        map.insert('2', vec!["a".to_string(),"b".to_string(),"c".to_string()]);
        map.insert('3', vec!["d".to_string(),"e".to_string(),"f".to_string()]);
        map.insert('4', vec!["g".to_string(),"h".to_string(),"i".to_string()]);
        map.insert('5', vec!["j".to_string(),"k".to_string(),"l".to_string()]);
        map.insert('6', vec!["m".to_string(),"n".to_string(),"o".to_string()]);
        map.insert('7', vec!["p".to_string(),"q".to_string(),"r".to_string(),"s".to_string()]);
        map.insert('8', vec!["t".to_string(),"u".to_string(),"v".to_string()]);
        map.insert('9', vec!["w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()]);
        let digits: Vec<char> = digits.chars().into_iter().collect();
        let mut letters = map.get(&digits[0]).unwrap().to_owned();
        for i in 1..digits.len() {
            letters = Solution::merge_sc(letters, map.get(&digits[i]).unwrap().to_owned());
        };
        letters
    }
}
```
