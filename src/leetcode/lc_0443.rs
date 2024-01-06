pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut new = String::new();
    let mut check = chars[0];
    let mut count = 1;
    for i in 1..chars.len() {
        if check == chars[i]{
            count += 1;
        } else {
            if count == 1 {
                new = format!("{}{}", new, check);
            } else {
                new = format!("{}{}{}", new, check, count);
            }
            check = chars[i];
            count = 1;
        }
    }
    if count == 1 {
        new = format!("{}{}", new, check);
    } else {
        new = format!("{}{}{}", new, check, count);
    }
    *chars = new.chars().collect();
    new.len() as i32    
}   