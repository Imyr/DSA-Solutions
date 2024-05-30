# 557. Reverse Words in a String III
- [Submission](https://leetcode.com/submissions/detail/1064061353/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.5 MB |
```
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ").map(|word| word.chars().rev().collect::<String>()).collect::<Vec<String>>().join(" ")
    }
}
```
- [Submission](https://leetcode.com/submissions/detail/1064030348/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Python3 | 48 ms | 16.8 MB |
```
class Solution:
    def reverseWords(self, s: str) -> str:
        return " ".join([word[::-1] for word in s.split(" ")])
```
