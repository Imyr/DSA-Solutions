pub fn reverse_str(s: String, k: i32) -> String {
    let k = k as usize;
    let n = s.len();
    let mut cursor = 0;
    if cursor+k > n {
        return s.chars().rev().collect();
    }
    let mut new = String::new();
    while cursor+k <= n {
        new += s[cursor..cursor+k].chars().rev().collect::<String>().as_str();
        cursor += k;
        if cursor+k <= n {
            new += s[cursor..cursor+k].chars().collect::<String>().as_str();
            cursor+=k;
        } else {
            new += s[cursor..n].chars().collect::<String>().as_str();
            cursor+=k;
        }
    }
    if cursor <= n {
        new += s[cursor..n].chars().rev().collect::<String>().as_str();
    }
    new
}