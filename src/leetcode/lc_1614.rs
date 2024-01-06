pub fn max_depth(s: String) -> i32 {   
    let mut max = 0;
    let mut size = 0;
    for c in s.chars() {
        match c {
            '(' => size+=1,
            ')' => size-=1,
            _ => {},
        }
        max = std::cmp::max(max, size)
    }  
    max
}