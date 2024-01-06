pub fn reverse(x: i32) -> i32 {
    let mut x = x;    
    let mut rev = 0;
    while x != 0 {
        if rev > std::i32::MAX/10 || rev < std::i32::MIN/10 {
            return 0;
        } 
        if x < 0 {
            if rev*10 + x%10 < std::i32::MIN {
                return 0;
            }
        } else {
            if rev*10 > std::i32::MAX - x%10 {
                return 0;
            }
        }
        rev = rev*10 + x%10;
        x/=10; 
        
    }
    return rev;
}