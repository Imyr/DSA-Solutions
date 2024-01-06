pub fn climb_stairs(n: i32) -> i32 {
    let n: u128 = n.try_into().unwrap();
    let (mut twos, mut ones) 
        = match n%2 {
            0 => (n/2, 0),
            1 => ((n-1)/2, 1),
            _ => panic!()
        };
    let mut ways = 0;
    while ones<=n {
        ways += (ones+1..=twos+ones).product::<u128>()/(1..=twos).product::<u128>();
        twos-=1;
        ones+=2;
    }
    ways as i32
}