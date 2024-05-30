# 1376. Time Needed to Inform All Employees
- [Submission](https://leetcode.com/submissions/detail/1249979740/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 51 ms | 13.3 MB |
```
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut tit = vec![];
        let manager = manager.into_iter().enumerate().fold(std::collections::HashMap::new(), 
            |mut acc, (i, x)| { if x!=-1 { acc.entry(x as usize).or_insert(vec![]).push(i) }; acc }
        );
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((head_id as usize, 0)); 
        while let Some((emp, cur_if)) = queue.pop_front() {
            if inform_time[emp] == 0 { tit.push(cur_if); continue; }
            for i in manager[&emp].iter() {
                queue.push_back((*i, cur_if + inform_time[emp]))
            }
        }
        tit.into_iter().max().unwrap()
    }
}
```
