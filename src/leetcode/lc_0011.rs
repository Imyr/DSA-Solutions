pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let (mut i, mut j) = (0, height.len()-1);
    while i<j {
        let (big, mut small) = (height[i], height[j]);
        if height[i] < height[j] {
            small = big;
        }
        let area = small*(j as i32 - i as i32);
        if area > max {
            max = area;
        }
        if height[i] < height[j] {
            i+=1;
        } else {
            j-=1;
        }
    }
    return max;
}