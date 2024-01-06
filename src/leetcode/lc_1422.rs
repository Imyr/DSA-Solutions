use std::collections::HashMap;

pub fn max_score(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let mut zeroes: HashMap<usize, i32> = HashMap::new();
    let mut ones: HashMap<usize, i32> = HashMap::new();
    
    let mut zero = 0;
    for i in 0..s.len() {
        if s[i] == '0' {
            zero += 1;
        }
        zeroes.insert(i, zero);
    }
    let mut one = 0;
    for i in (0..s.len()).rev() {
        ones.insert(i, one);
        if s[i] == '1' {
            one += 1;
        }
    }
    println!("{:?} {:?}", zero, one);
    let mut max = i32::MIN;
    for i in 0..s.len()-1 {
        if zeroes[&i]+ones[&i] > max {
            max = zeroes[&i]+ones[&i];
        }
    }
    max
}