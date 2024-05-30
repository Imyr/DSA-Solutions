# 402. Remove K Digits
- [Submission](https://leetcode.com/submissions/detail/1229806836/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.5 MB |
```
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut ms = vec![];
        let mut del = k as usize;
        for c in num.chars() {
            while !ms.is_empty() && *ms.last().unwrap() > c && del > 0 {
                ms.pop();
                del -= 1;
            }
            ms.push(c);
        }
        ms.truncate(ms.len() - del);
        let res = ms.into_iter().collect::<String>().trim_start_matches(|x| x == '0').to_string();
        if res.is_empty() { "0".to_string() } else { res }
    }
}
```
