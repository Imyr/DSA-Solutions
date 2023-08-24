use std::collections::HashSet;

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let prepresum = target as i64 - (nums[i] as i64+nums[j] as i64);
            for k in j+1..nums.len() {
                let presum = prepresum - nums[k] as i64;
                if presum > i32::MAX as i64 || presum < i32::MIN as i64{
                    continue;
                }
                match nums.binary_search(&(presum as i32)) {
                    Ok(idx) => {
                        if idx!=i && idx!=j && idx!=k {
                        let mut hmm = vec![nums[i], nums[j], nums[k], nums[idx]];
                        hmm.sort_unstable();
                        set.insert(hmm);
                        }   
                    },
                    Err(_) => continue,
                };
            }
        }
    }
    set.into_iter().collect()
}