use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut kek: HashMap<i32, usize> = HashMap::new();
    for i in 0..arr.len() {
        if kek.contains_key(&arr[i]) {
            kek.insert(arr[i], kek[&arr[i]] + 1);
        } else {
            kek.insert(arr[i], 1);
        }
    }
    let bruh: HashSet<&usize> = HashSet::from_iter(kek.values());
    if kek.len() == bruh.len() {
        return false;
    } else {
        return true;
    }
}