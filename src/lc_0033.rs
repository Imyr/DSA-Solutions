pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut m;
    let mut r = nums.len()-1;
    while l < r {
        
        m = (l+r)/2;
        println!("l: {}, r: {}", l, r);
        if nums[m] > nums[r] {
            print!("yeah");
            l = m+1;
        } else {
            r = m-1;
        }
    }
    todo!()
}