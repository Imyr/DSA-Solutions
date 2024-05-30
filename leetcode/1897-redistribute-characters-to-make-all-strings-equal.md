# 1897. Redistribute Characters to Make All Strings Equal
- [Submission](https://leetcode.com/submissions/detail/1136855731/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.3 MB |
```
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let x = words.iter().map(
            |word| {
                let mut count = vec![0; 26];
                for &c in word.as_bytes() { count[c as usize - 97] += 1 }
                count
            }   
        ).collect::<Vec<Vec<usize>>>();
        (0..26).collect::<Vec<usize>>().iter().map(|i| x.iter().map(|y| y[*i]).sum::<usize>()).all(|z| z==0 || z%words.len()==0)
    }
}
```
