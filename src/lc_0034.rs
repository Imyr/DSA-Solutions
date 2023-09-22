pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    match nums.binary_search(&target) {
        Ok(idx) => {
            let mut low = idx;
            let mut high = idx;
            for i in (0..idx).rev() {
                if nums[i] == target {
                    low = i;
                } else {
                    break;
                }
            }
            for i in idx+1..nums.len() {
                if nums[i] == target {
                    high = i;
                } else {
                    break;
                }
            }
            return vec![low as i32, high as i32];
        },
        Err(_) => return vec![-1, -1],
    }
}