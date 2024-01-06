pub fn is_power_of_two(n: i32) -> bool {
    match n {
        i32::MIN..=-1 => return false,
        0..=i32::MAX => {
            if i32::count_ones(n) == 1 {
                return true;
            }
            return false;
        }
    }
}