pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut yum = vec![];
    for i in (1..nums.len()).rev() {
        if nums[i]==nums[i-1] {
            yum.push(nums[i]);
        }
    }
    yum
}