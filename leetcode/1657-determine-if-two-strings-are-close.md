# 1657. Determine if Two Strings Are Close
- [Submission](https://leetcode.com/submissions/detail/1146263031/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 2.3 MB |
```
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut count1 = word1.as_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut count, x| {
                count[*x as usize - 97] += 1;
                count
        });
        let mut count2 = word2.as_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut count, x| {
                count[*x as usize - 97] += 1;
                count
        });
        for i in 0..26 {
            if (count1[i]==0&&count2[i]!=0) || (count1[i]!=0&&count2[i]==0) {
                return false;
            }
        }
        count1.sort_unstable();
        count2.sort_unstable();       
        count1 == count2
    }
}
```
