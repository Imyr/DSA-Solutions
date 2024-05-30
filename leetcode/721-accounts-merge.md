# 721. Accounts Merge
- [Submission](https://leetcode.com/submissions/detail/1268827021/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 751 ms | 4.2 MB |
```
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut accounts = accounts.into_iter().fold(HashMap::new(), |mut acc: HashMap<String, Vec<HashSet<String>>>, account| {
            acc.entry(account[0].clone()).or_insert(vec![]).push(account.into_iter().skip(1).collect()); acc
        });
        let mut result = vec![];
        for (name, acc) in accounts.iter_mut() {
            let mut res = vec![];
            while let Some(mut mails) = acc.pop() {
                if acc.is_empty() { res.push(mails); continue; }
                let mut i = acc.len()-1;
                while i >= 0 {
                    if !mails.is_disjoint(&acc[i]) { 
                        mails = mails.union(&acc.remove(i)).map(|x| x.clone()).collect();
                        i = acc.len();
                    }
                    if i == 0 { break; }
                    i -= 1;
                }
                res.push(mails);
            }
            for x in res {
                let mut bruh = vec![name.clone()];
                let mut y: Vec<String> = x.into_iter().collect();
                y.sort_unstable();
                bruh.append(&mut y);
                result.push(bruh);
            }
        }
        result
    }
}
```
