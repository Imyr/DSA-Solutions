pub fn final_string(s: String) -> String {
    let mut hmm = String::new();
    for c in s.chars() {
        if c == 'i' {
            hmm = hmm.chars().rev().collect();
        } else {
            hmm += c.to_string().as_str();
        }
    }
    hmm
}