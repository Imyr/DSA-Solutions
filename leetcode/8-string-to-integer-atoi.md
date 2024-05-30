# 8. String to Integer (atoi)
- [Submission](https://leetcode.com/submissions/detail/981683617/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 2 ms | 2.1 MB |
```
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut num = 0;
        let s = s.trim().to_string();
        let mut kek = vec![];
        for hmm in s.chars() {
            kek.push(hmm);
        };
        if kek.len() == 0 {
            return 0;
        }
        if kek.len() == 1 {
            match kek[0] as i32 {
            48..=57 => return kek[0] as i32 - 48,
            _ => return 0,
            }   
        }
        let mut i = 0;
        let mut sign = 1;
        match kek[0] {
            '+' => {
                i = 1;
            },
            '-' => {
                i = 1;
                sign = -1;
            },
            _ => {},
        }
        while 48 <= kek[i] as i32 && kek[i] as i32 <= 57 {
            let chara = kek[i] as i32 - 48;
            if sign == 1 {
                if num > i32::MAX/10 {
                    return i32::MAX;
                } 
                if num*10 > i32::MAX - chara {
                    return i32::MAX;
                }
                num = num*10 + chara;
                i += 1;
            } else {
                if num - 1 > i32::MAX/10 {
                    return i32::MIN;
                } 
                if num*10 - 1 > i32::MAX - chara {
                    return i32::MIN;
                }
                if (num*10 - 1) + chara == i32::MAX{
                    num = -1*num*10 - chara;
                    return num;
                }
                num = num*10 + chara;
                i += 1;
            }
            if i >= kek.len() {
                break;
            }
        }
        return num*sign;
    }
}
```
