pub fn integer_break(n: i32) -> i32 {
    let mut max = 0;
    let mut i = 2;
    while n!=i {
        let mut temp = n;
        let mut prod = 1;
        while temp>2*i {
            temp -= i;
            prod *= i;
        }
        if i*(temp-i) > temp {
            prod *= i*(temp-i);
        } else {
            prod *= temp;
        }
        if max <= prod {
            max = prod;
        } else {
            break;
        }
        i+=1;
    }
    max
}


// pub fn integer_break(n: i32) -> i32 {
//     let mut max = 0;
//     for i in 2..i32::MAX {
//         let q = n/i;
//         if q==0 {
//             break;
//         }
//         let mut r = n%i;
//         println!("i:{}, q:{}, r:{}", i, q, r);
//         for _ in 0..q {
//             r*=i;
//         }
//         println!("{} {}", max, r);
//         if max<r {
//             max = r;
//         } 
//         // else {
//         //     break;
//         // }
//     }
//     max
// }