# 661. Image Smoother
- [Submission](https://leetcode.com/submissions/detail/1123267231/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 13 ms | 2.2 MB |
```
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut smooth = vec![vec![0; img[0].len()]; img.len()];
        let (im, jm) = (img.len() as i32, img[0].len() as i32);
        for i in 0..img.len() as i32 {
            for j in 0..img[0].len() as i32 {
                let mut total = 0;
                let mut div = 0;
                for x in i-1..=i+1 {
                    for y in j-1..=j+1 {
                        if x >= 0 && x < im && y >= 0 && y < jm {
                            total += img[x as usize][y as usize];
                            div += 1;
                        }
                    }
                }
                smooth[i as usize][j as usize] = total/div;
            }
        }
        smooth
    }
}
```
