use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    for n in nums { *map.entry(n).or_insert(0) += 1 }
    let mut twod = vec![];
    let mut flag = true;
    while flag {
        flag = false;
        let mut v = vec![]; 
        for (val, times) in map.iter() {
            if times > &0 { 
                v.push(*val); 
                flag = true;
            }
        }
        for x in v.iter() { *map.get_mut(x).unwrap() -= 1; }
        if flag { twod.push(v)}
    }
    twod     
}