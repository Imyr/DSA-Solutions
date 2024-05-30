# 1255. Maximum Score Words Formed by Letters
- [Submission](https://leetcode.com/submissions/detail/1266552915/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 3 ms | 2.1 MB |
```
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let letters = letters.into_iter().fold(vec![0; 26], |mut acc, c| { acc[c as usize - 'a' as usize] += 1; acc });
        let n = words.len();
        (0..2i32.pow(n as u32)).map(|b| {
            let mut s = 0;
            let mut templ = letters.clone();
            let mut brk = false;
            for i in 0..n {
                if b&1<<i>0 {
                    for j in words[i].chars() {
                        let c = j as usize - 'a' as usize;
                        if templ[c] > 0 { templ[c] -= 1; }
                        else { brk = true; s = 0; break; }
                        s += score[c];
                    }
                }
                if brk { break; }
            }
            s
        }).into_iter().max().unwrap()
    }
}
```
