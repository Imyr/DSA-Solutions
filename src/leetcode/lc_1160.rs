use std::collections::HashMap;

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut map: HashMap<char, u8> = HashMap::new();
    for c in chars.chars() {
        *map.entry(c).or_insert(0)+=1;
    }
    let mut count = 0;
    for word in words {
        let mut check = map.clone();
        let mut yes = 0;
        for c in word.chars() {
            match check.get_mut(&c) {
                Some(0) => {yes=-1; break;},
                None => {yes=-1; break;},
                Some(x) => *x-=1,
            }
        }
        if yes==-1 {
            continue;
        } else {
            count+=word.len() as i32;
        }
    }
    count
}