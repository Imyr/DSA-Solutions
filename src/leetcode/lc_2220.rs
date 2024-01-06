pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
    let mut flip = 0;
    while start!=0 || goal!=0 {
        if start&1 != goal&1 {
            flip += 1;
        }
        start>>=1;
        goal>>=1;
    }
    flip
}