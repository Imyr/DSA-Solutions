pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut cursor = nums.len()-1;
    for i in (0..nums.len()-1).rev() {
        if nums[i]==0 {
            nums[i] = nums[cursor];
            nums[cursor] = 0;
            cursor-=1;
        }
    }
}