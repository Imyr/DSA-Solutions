trait Search {
    fn custom_bsi(&self, target: &i32, l: usize, r: usize) -> usize ;
}

impl Search for Vec<i32> {
    fn custom_bsi(&self, target: &i32, mut l: usize, mut r: usize) -> usize {
        let mut m;
        while l <= r {
            m = l + (r - l) / 2;
            if target < &self[m] {
                if m > 0 { 
                    r = m - 1;
                } else {
                    return m;
                }
            } else if &self[m] < target {
                if m < self.len() {
                    l = m + 1;
                } else {
                    return m;
                }
            } else {
                return m;
            }
        }
        return l;
    }
} 

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.custom_bsi(&target, 0, nums.len()-1) as i32
}







// trait Search {
//     fn custom_bsi(&self, target: &i32, l: usize, r: usize) -> Result<usize, usize> ;
// }

// impl Search for Vec<i32> {
//     fn custom_bsi(&self, target: &i32, mut l: usize, mut r: usize) -> Result<usize, usize> {
//         let mut m;
//         while l <= r {
//             m = l + (r - l) / 2;
//             if target < &self[m] {
//                 if m > 0 { 
//                     r = m - 1;
//                 } else {
//                     return Err(m);
//                 }
//             } else if &self[m] < target {
//                 if m < self.len() {
//                     l = m + 1;
//                 } else {
//                     return Err(m);
//                 }
//             } else {
//                 return Ok(m);
//             }
//         }
//         return Err(l);
//     }
// } 

// pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//     match nums.custom_bsi(&target, 0, nums.len()-1) {
//         Ok(idx) => idx as i32,
//         Err(idx) => idx as i32,
//     }    
// }