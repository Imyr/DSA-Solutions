use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len()/3;
    let mut unique = Vec::new();
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        match counter.get_mut(&num) {
            Some(x) => {*x+=1},
            None => {counter.insert(num, 1);},
        }
    }
    for (number, repetitions) in counter {
        if repetitions > n as i32 {
            unique.push(number);
        } 
    }
    unique
}