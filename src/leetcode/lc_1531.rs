use std::collections::HashMap;

pub fn get_length_of_optimal_compression(s: String, mut k: i32) -> i32 {
    let mut new = String::new();
    let mut map = HashMap::new();

    let s = s.chars().collect::<Vec<char>>();
    let mut check = s[0];
    let mut count = 1;
    for i in 1..s.len() {
        if check == s[i]{
            count += 1;
        } else {
            if count == 1 {
                new = format!("{}{}", new, check);
            } else {
                new = format!("{}{}{}", new, check, count);
            }
            *map.entry(count).or_insert(0) += 1;
            check = s[i];
            count = 1;
        }
    }
    if count == 1 {
        new = format!("{}{}", new, check);
    } else {
        new = format!("{}{}{}", new, check, count);
    }
    *map.entry(count).or_insert(0) += 1;

    println!("{}", new);
    println!("{:?}", map);
    
    let mut total = new.len();
    for (&(count), &(ch)) in map.iter() {
        for _ in 1..=ch {
            for _ in 1..=count {
                if k != 0 {
                    k -= 1;
                } else {
                    return total as i32;
                }  
            }
            total -= 1;
        }
    }
    unreachable!()   
}