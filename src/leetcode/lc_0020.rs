pub fn is_valid(s: String) -> bool {
    let mut hmm = vec![];
    for c in s.chars() {
        match c {
            '(' => hmm.push(c),
            ')' => {
                if hmm.pop()!=Some('(') {
                    return false;
                }
            },
            '{' => hmm.push(c),
            '}' => {
                if hmm.pop()!=Some('{') {
                    return false;
                }
            },
            '[' => hmm.push(c),
            ']' => {
                if hmm.pop()!=Some('[') {
                    return false;
                }
            },
            _ => continue,
        }
    }
    hmm.len()==0
}