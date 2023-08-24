pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let first: Vec<char> = strs[0].chars().collect();
    
    let mut i: usize = 0;
    loop {
        if i<first.len() {
            let c = first[i];
            for idx in 0..strs.len() {
                let s: Vec<char> = strs[idx].chars().collect();
                if i>=s.len() {
                    return prefix;
                } else {
                    if c!=s[i] {
                        return prefix;
                    }
                }
            }
            prefix = format!("{}{}", prefix, c);
        } else {
            return prefix;
        }
        i+=1;
    }
}