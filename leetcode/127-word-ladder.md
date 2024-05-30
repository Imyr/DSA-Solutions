# 127. Word Ladder
- [Submission](https://leetcode.com/submissions/detail/1256274486/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 182 ms | 7.3 MB |
```
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let n = begin_word.len();
        let s = begin_word.chars().collect::<Vec<_>>();
        let e = end_word.chars().collect::<Vec<_>>();
        let mut word_list = word_list.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        word_list.sort_unstable();
        let mut queue = (0..n).into_iter().map(|i| (0, i, s.clone())).collect::<std::collections::VecDeque<_>>();
        let mut visited = std::collections::HashSet::new();
        while let Some((depth, i, w)) = queue.pop_front() {
            if w == e { return depth+1; }
            if !visited.insert((i, w.clone())) { continue; }
            for c in 'a'..='z' {
                if c != w[i] { 
                    let mut nw = w.clone();
                    nw[i] = c;
                    if word_list.binary_search(&nw).is_ok() {
                        for j in 0..n {
                            queue.push_back((depth+1, j, nw.clone()));
                        }
                    }
                }
            }
        }
        0
    }
}
```
