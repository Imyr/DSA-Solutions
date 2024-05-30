# 1578. Minimum Time to Make Rope Colorful
- [Submission](https://leetcode.com/submissions/detail/1130009376/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 3.4 MB |
```
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.chars().collect::<Vec<char>>();
        let mut total = needed_time[0];
        let mut max = needed_time[0];
        let mut check = colors[0];
        for i in 1..colors.len() {
            if check == colors[i] {
                if max < needed_time[i] {
                    max = needed_time[i];
                }
                total += needed_time[i];
            } else {
                total -= max;
                check = colors[i];
                max = needed_time[i];
                total += max;
            }
        }
        total -= max;
        total       
    }
}
```
