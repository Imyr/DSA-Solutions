use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *counter.entry(num).or_insert(0) += 1;
    }
    *counter.iter().max_by_key(|&(_, rep)| rep).unwrap().0
}