pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len()-1;
    let mut m;
    while l <= r {
        m = l + (r - l) / 2;
        if m>0 && nums[m-1]==nums[m] {
            if (m-1)%2==0 {
                l = m + 1;
            } else {
                r = m - 2;
            }
        } else if m<nums.len()-1 && nums[m]==nums[m+1] {
            if (m)%2==0 {
                l = m + 2;
            } else {
                r = m - 1;
            }
        } else {
            return nums[m]
        }
    }
    unreachable!()
}