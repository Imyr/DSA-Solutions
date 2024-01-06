use std::collections::HashMap;

pub fn beauty_sum(s: String) -> i32 {
    let mut table: HashMap<char, u16> = HashMap::new();
    for c in s.chars() {
        *table.entry(c).or_insert(0)+=1;
    }
    let max = table.iter().max_by_key(|(_, k)| *k);
    let min = table.iter().min_by_key(|(_, k)| *k);
    println!("max: {:?}, min: {:?}", max, min);
    -1   
}