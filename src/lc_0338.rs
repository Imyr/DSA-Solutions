// pub fn count_bits(n: i32) -> Vec<i32> {
//     let mut bits = vec![];
//     for i in 0..=n {
//         bits.push(i.count_ones() as i32);
//     }
//     bits
// }

pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|mut x| {
        let mut count: i32 = 0;
        while x!=0 {
            count += x&1;
            x>>=1;
        }
        count
    }).collect()
}