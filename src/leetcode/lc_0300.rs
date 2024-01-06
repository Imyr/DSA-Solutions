fn llis(i: usize, seq: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
    println!("{:?}", cache);
    if i >= seq.len() {
        0
    } else if cache[i] != 0 {
        cache[i]
    } else {
        let mut sub = vec![];
        for j in i+1..seq.len() {
            if seq[i] < seq[j] {
                sub.push(1 + llis(j, seq, cache))
            }
        } 
        cache[i] = *sub.iter().max().unwrap_or(&1);
        cache[i]
    }
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut cache = vec![0; nums.len()];
    for i in 0..nums.len() {
        println!("lis: {}", llis(i, &nums, &mut cache));
    }
    *cache.iter().max().unwrap()
}