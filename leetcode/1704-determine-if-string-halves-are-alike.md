# 1704. Determine if String Halves Are Alike
- [Submission](https://leetcode.com/submissions/detail/1144524372/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.2 MB |
```
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        s.split_at(s.len()/2).0.chars()
        .filter(|x| match *x {
            'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }).count()
        == s.split_at(s.len()/2).1.chars()
        .filter(|x| match *x {
            'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }).count()
    }
}
```
