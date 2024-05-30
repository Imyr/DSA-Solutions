# 68. Text Justification
- [Submission](https://leetcode.com/submissions/detail/1255587693/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.2 MB |
```
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max = max_width as usize;
        let mut i = 0;
        let mut justified = vec![];
        while i < words.len() {
            if words[i].len() == max { justified.push(words[i].clone()); i += 1; continue;}
            let mut j = i;
            let mut total = 0;
            let mut words_len = 0;
            while total <= max && j < words.len() {
                if total + words[j].len() == max { 
                    total += words[j].len();
                } else if total + words[j].len() < max{
                    total += words[j].len() + 1;
                } else {
                    break;
                }
                words_len += words[j].len();
                j += 1;
            }
            let mut line = vec![];
            for k in i..j { line.push(&words[k]); }
            let mut new_line = String::new();

            if line.len() != 1 {
                if j >= words.len() {
                    for i in 0..line.len()-1 {
                        new_line += line[i];
                        if new_line.len() < max { new_line += &" "; }
                    }
                    new_line += line[line.len()-1];
                    new_line += &" ".repeat(max - new_line.len());
                } else {
                    let extra_spaces = max - (words_len + line.len()-1);
                    let mut spaces = vec![1+(extra_spaces/(line.len()-1)); line.len()-1];
                    let mut extra_spaces = extra_spaces%(line.len()-1);
                    for i in 0..spaces.len() {
                        if extra_spaces <= 0 { break; }
                        spaces[i] += 1;
                        extra_spaces -= 1;
                    }
                    for i in 0..line.len()-1 {
                        new_line += line[i];
                        new_line += &" ".repeat(spaces[i]);
                    }
                    new_line += line[line.len()-1];
                }
            } else {
                let mut spaces = max - line[0].len();
                new_line += line[0];
                new_line += &" ".repeat(spaces);
            }
            justified.push(new_line);
            i = j;
        }
        justified
    }
}
```
