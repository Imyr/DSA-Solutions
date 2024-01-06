use std::collections::HashMap;

pub fn decode(s: String, m: &mut HashMap<String, i32>) -> i32 {
    if s.starts_with('0') {
        return 0;
    } else if s.len() == 1 || s.len() == 0 {
        return 1;
    } else if let Some(x) = m.get(&s) {
        return *x
    } else {
        let s = s.chars().collect::<Vec<char>>();
        let this = s[0].to_digit(10).unwrap();
        let next = s[1].to_digit(10).unwrap();
        let val = if this == 1 {
            decode(s[1..].iter().collect::<String>(), m) + decode(s[2..].iter().collect::<String>(), m)
        } else if this == 2 {
            if next <= 6 {
                decode(s[1..].iter().collect::<String>(), m) + decode(s[2..].iter().collect::<String>(), m)
            } else {
                decode(s[1..].iter().collect::<String>(), m)
            }
        } else {
            decode(s[1..].iter().collect::<String>(), m)
        };
        m.insert(s.iter().collect::<String>(), val);
        val
    }
}

pub fn num_decodings(s: String) -> i32 {
    let mut hehe = HashMap::new();
    decode(s, &mut hehe)
}