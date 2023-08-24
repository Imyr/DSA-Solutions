pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut diff = target-(nums[0]+nums[1]+nums[2]);
    let mut min = nums[0]+nums[1]+nums[2];
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let presum = target-(nums[i]+nums[j]);
            match nums.binary_search(&presum) {
                Ok(idx) => {
                    if idx!=i && idx!=j {
                        min = nums[i]+nums[j]+nums[idx];
                        diff = target-min;
                    }    
                },
                Err(mut idx) => {
                    if idx==nums.len() || (idx!=0 && nums[idx]<0) {
                        idx-=1;
                    }
                    if idx!=i && idx!=j && diff.abs()>(presum-nums[idx]).abs() {
                        min = nums[i]+nums[j]+nums[idx];
                        diff = target-min;

                    }
                },
            }
        }
    }
    min
}