pub fn fib(n: i32) -> i32 {
    let (mut one,  mut two) = (0, 1);
    for _ in 2..=n as usize{
        two = one + two;
        one = two - one;
    }
    two
}