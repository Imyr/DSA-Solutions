use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut sorted = String::new();

    let mut table: HashMap<char, u16> = HashMap::new();
    for c in s.chars() {
        *table.entry(c).or_insert(0)+=1;
    }
    let mut list: Vec<_> = table.iter().collect();
    println!("{:?}", list);
    list.sort_by(|(_, a), (_, b)| (*b).cmp(*a));
    println!("{:?}", list);
    for (c, i) in list {
        sorted.push_str(c.to_string().repeat(*i as usize).as_str());
    }

    sorted     
}