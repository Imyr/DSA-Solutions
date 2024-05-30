# 4. Median of Two Sorted Arrays
- [Submission](https://leetcode.com/submissions/detail/1055603124/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 0 ms | 2.3 MB |
```
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (big, small) = match nums1.len() >= nums2.len()  {
            true => (nums1, nums2),
            false => (nums2, nums1)
        };
        let n = big.len() as i32 + small.len() as i32;
        let mut cursor = 0; let mut first = 0; let mut second = 0;
        let position = match n%2==0 {
            true => n/2 ,
            false => (n-1)/2
        };
        let mut i = 0; let mut j = 0;
        while j != small.len() && cursor != position+1 {
            if i == big.len() && j != small.len() {
                first = second;
                second = small[j];
                j+=1;
                cursor += 1;
                continue;
            } 
            if big[i] < small[j] {
                first = second;
                second = big[i];
                i += 1;
                cursor += 1;
                continue;
            } 
            if big[i] > small[j] {
                first = second;
                second = small[j];
                j += 1;
                cursor += 1;
                continue;
            }
            if big[i] == small[j] {
                first = second;
                second = big[i];
                cursor+=1;
                if cursor == position+1 {
                    break;
                }
                first = small[j];
                i+=1; j+=1;
                cursor += 1;
                continue;
            }
            
        };
        while i != big.len() && cursor != position+1 {
            first = second;
            second = big[i];
            i += 1;
            cursor += 1;
        };
        match n%2==0 {
            true => (first as f64 + second as f64)/2.0,
            false => second as f64
        }
    }
}
```
