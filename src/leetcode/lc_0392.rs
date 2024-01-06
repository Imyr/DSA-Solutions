pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s = s.chars();
    let mut check = match s.next() {
        Some(c) => c,
        None => return true,
    };

    for c in t.chars() {
        if c==check {
            check = match s.next() {
                Some(c) => c,
                None => return true,
            }
        }
    }
    false
}