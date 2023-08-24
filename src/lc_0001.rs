use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut bruh = HashMap::new();
    bruh.insert(nums[0], 0 as usize);
    for i in 1..nums.len() {
        let diff = target - nums[i];
        if !(bruh.contains_key(&diff)) {
            bruh.insert(nums[i], i);
            println!("{:?}", bruh);
        } else {
            return vec![bruh[&diff] as i32, i as i32]
        }
    }
    return vec![-1, -1]
}