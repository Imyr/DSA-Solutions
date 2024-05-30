# 1287. Element Appearing More Than 25% In Sorted Array
- [Submission](https://leetcode.com/submissions/detail/1117046758/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.2 MB |
```
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut temp = arr[0];
        let mut count = 1;
        for i in 1..arr.len() {
            if count > arr.len()/4 {
                return temp
            }
            if arr[i] != temp {
                temp = arr[i];
                count = 1;
            } else {
                count += 1;
            }
        }   
        temp 
    }
}
```
