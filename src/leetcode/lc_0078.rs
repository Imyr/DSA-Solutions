pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut list = vec![];
    for bin in 0..2i32.pow(nums.len() as u32) {
        let mut temp_list = vec![];
        for i in 0..nums.len() {    
            if (bin>>i)&1 == 1 {
                temp_list.push(nums[i]);
            }
        }
        list.push(temp_list);
    }
    list
}