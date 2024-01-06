// trait MBS {
//     fn my_binary_search(&self, target: &i32, l: usize, r: usize) -> Result<usize, ()> ;
// }

// impl MBS for Vec<i32> {
//     fn my_binary_search(&self, target: &i32, mut l: usize, mut r: usize) -> Result<usize, ()> {
//         let mut m;
//         while l <= r {
//             m = l + (r - l) / 2;
//             if target < &self[m] {
//                 r = m - 1;
//             } else if &self[m] < target {
//                 l = m + 1;
//             } else {
//                 return Ok(m);
//             }
//         }
//         return Err(());
//     }
// } 



pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_index = -1;
    let mut r = nums.len();
    while left_index==-1 {
        match nums[0..r].binary_search(&target) {
            Err(_) => return vec![-1, -1],
            Ok(idx) => {
                let idx = idx as usize;
                if idx==0 || nums[idx-1]!=target {
                    left_index = idx as i32;
                } else {
                    r = idx;
                }
            },
        } 
    }

    let mut right_index = -1;
    let mut l = 0;
    while right_index==-1 {
        match nums[l..nums.len()].binary_search(&target) {
            Err(_) => unreachable!(),
            Ok(idx) => {
                if l+idx+1==nums.len() || nums[l+idx+1]!=target {
                    right_index = (l+idx) as i32;
                } else {
                    l = l+idx+1;
                }
            },
        } 
    }

    vec![left_index, right_index]
}






// pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     if nums.len()==0 {
//         return vec![-1, -1]
//     }
//     if nums.len()==1 {
//         if 
//     }

//     let mut left_index = -1;
//     let mut r = nums.len();
//     while left_index==-1 {
//         match nums.my_binary_search(&target, 0, r-1) {
//             Err(_) => return vec![-1, -1],
//             Ok(idx) => {
//                 let idx = idx as usize;
//                 if idx==0 || nums[idx-1]!=target {
//                     left_index = idx as i32;
//                 } else {
//                     r = idx;
//                 }
//             },
//         } 
//     }

//     let mut right_index = -1;
//     let mut l = 0;
//     while right_index==-1 {
//         match nums.my_binary_search(&target, l, nums.len()-1) {
//             Err(_) => unreachable!(),
//             Ok(idx) => {
//                 if idx+1==nums.len() || nums[idx+1]!=target {
//                     right_index = idx as i32;
//                 } else {
//                     l = idx;
//                 }
//             },
//         } 
//     }

//     vec![left_index, right_index]
// }








// pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     match nums.binary_search(&target) {
//         Ok(idx) => {
//             let mut low = idx;
//             let mut high = idx;
//             for i in (0..idx).rev() {
//                 if nums[i] == target {
//                     low = i;
//                 } else {
//                     break;
//                 }
//             }
//             for i in idx+1..nums.len() {
//                 if nums[i] == target {
//                     high = i;
//                 } else {
//                     break;
//                 }
//             }
//             return vec![low as i32, high as i32];
//         },
//         Err(_) => return vec![-1, -1],
//     }
// }