use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for x in nums { *map.entry(x).or_insert(0usize) += 1 }
    let mut ops = 0;
    let rem = vec![0, 1, 1, 1, 2, 2];
    for y in map.into_values() {
        if y == 1 {
            return -1;
        } else {
            ops += y/3 + rem[y%6];
        }
    }
    ops as i32
}