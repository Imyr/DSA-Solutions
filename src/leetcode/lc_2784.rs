pub fn is_good(mut nums: Vec<i32>) -> bool {
    nums.sort();
    if nums.len() != *nums.last().unwrap() as usize + 1 {
        return false
    }
    for i in 0..nums.len()-1 {
        if nums[i] != i as i32 + 1 {
            return false
        }
    }
    true 
}