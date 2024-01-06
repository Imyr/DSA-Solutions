pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len()==1 {
        if nums[0]!=target {
            return -1;
        } else {
            return 0;
        }
    }

    if nums[0] < nums[nums.len()-1] {
        match nums.binary_search(&target) {
            Ok(idx) => return idx as i32,
            Err(_) => return -1,
        }
    }

    let mut l = 0;
    let mut r = nums.len()-1;
    let mut m;
    let mut p = 0;
    while l<=r {
        println!("l: {}, r: {}", l, r);
        if l==r {
            p = l;
            break;
        }
        m = (l+r)/2;
        if (m>0 && nums[m]<nums[m-1]) && (m<nums.len()-1 && nums[m]<nums[m+1]) {
            p = m;
            break;
        }
        if nums[m] > nums[r] {
            l = m+1;
        } else {
            r = m-1;
        }
    }

    if nums[p]==target {
        return p as i32;
    }

    if nums[0]==target {
        return p as i32;
    }

    if nums[0]>target {
        l = p;
        r = nums.len()-1;
    } else {
        l = 0;
        r = p-1;
    }

    println!("{}", p);

    
    println!("{:?}", &nums[l..=r]);
    match nums[l..=r].binary_search(&target) {
        Ok(idx) => {
            if l==0{
                idx as i32
            } else {
                (p+idx) as i32
            }
        },
        Err(_) => -1,
    }
}



// let mut l = 0;
// let mut m;
// let mut r = nums.len()-1;
// while l < r {
    
//     m = (l+r)/2;
//     println!("l: {}, r: {}", l, r);
//     if nums[m] > nums[r] {
//         print!("yeah");
//         l = m+1;
//     } else {
//         r = m-1;
//     }
// }
// todo!()