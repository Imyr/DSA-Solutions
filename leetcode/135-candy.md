# 135. Candy
- [Submission](https://leetcode.com/submissions/detail/1114408308/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1737 ms | 2.2 MB |
```
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        match ratings.len() {
            1 => 1,
            2 => {
                if ratings[0] == ratings[1] {
                    2
                } else {
                    3
                } 
            }, 
            _ => {
                let mut candies = vec![1; ratings.len()];
                let mut run = true;
                while run {
                    run = false;
                    if ratings[0] > ratings[1] {
                        if candies[0] <= candies[1] {
                            candies[0] += 1;
                            run = true;
                        }
                    }
                    for i in 1..=ratings.len()-2 {
                        if ratings[i-1] < ratings[i] {
                            if candies[i-1] >= candies[i] {
                                candies[i] += 1;
                                run = true;
                            }
                        }
                        if ratings[i] > ratings[i+1] {
                            if candies[i] <= candies[i+1] {
                                candies[i] += 1;
                                run = true;
                            }
                        }
                    }
                    if ratings[ratings.len()-2] < ratings[ratings.len()-1] {
                        if candies[ratings.len()-2] >= candies[ratings.len()-1] {
                            candies[ratings.len()-1] += 1;
                            run = true;
                        }
                    }
                }
                candies.iter().sum()
            }
        } 
    }
}
```
