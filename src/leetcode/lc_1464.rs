pub fn max_product(nums: Vec<i32>) -> i32 {
    let (mut i1, mut i2) = (0, 0);       
    for e in nums {
        if e >= i1 {
            i2 = i1;
            i1 = e;
        } else if e > i2 {
            i2 = e;
        }
    }
    (i1-1)*(i2-1)
}