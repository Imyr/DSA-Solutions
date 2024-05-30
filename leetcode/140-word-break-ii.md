# 140. Word Break II
- [Submission](https://leetcode.com/submissions/detail/1267215891/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.1 MB |
```
impl Solution {
    fn dp(i: usize, s: &String, word_dict: &Vec<String>, sentence: Vec<String>, result: &mut Vec<String>) {
        if i >= s.len() { return; }
        for word in word_dict.iter() {
            if i + word.len() == s.len() && &s[i..i+word.len()] == word.as_str() {
                let mut temp_sentence = sentence.clone();
                temp_sentence.push(word.to_string());
                result.push(temp_sentence.join(" "));
            } else if i + word.len() < s.len() && &s[i..i+word.len()] == word.as_str() {
                let mut temp_sentence = sentence.clone();
                temp_sentence.push(word.to_string());
                Self::dp(i + word.len(), s, word_dict, temp_sentence, result);
            }  
        }
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        Self::dp(0, &s, &word_dict, vec![], &mut result);
        result
    }
}
```
