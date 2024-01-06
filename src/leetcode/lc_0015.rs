use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let presum = -(nums[i]+nums[j]);
            match nums.binary_search(&presum) {
                Ok(idx) => {
                    if idx!=i && idx!=j {
                    let mut hmm = vec![nums[i], nums[j], nums[idx]];
                    hmm.sort_unstable();
                    set.insert(hmm);
                }},
                Err(_) => continue,
            };
        }
    }
    set.into_iter().collect()
}