pub fn add_binary(a: String, b: String) -> String {
    let mut lul = String::new();
    let a: Vec<char> = a.chars().rev().into_iter().collect();
    let b: Vec<char> = b.chars().rev().into_iter().collect();
    let mut carry = 0;
    for i in 0..std::cmp::max(a.len(), b.len()) {
        match (a.get(i).unwrap_or_else(|| &'0'), b.get(i).unwrap_or_else(|| &'0')) {
            ('0', '1') | ('1', '0') => {
                if carry == 1 {
                    lul = format!("{}{}", 0, lul);
                } else {
                    lul = format!("{}{}", 1, lul);
                }
            },
            ('0', '0') => {
                if carry == 1 {
                    lul = format!("{}{}", 1, lul);
                    carry = 0;
                } else {
                    lul = format!("{}{}", 0, lul);
                }
            },
            ('1', '1') => {
                if carry == 1 {
                    lul = format!("{}{}", 1, lul);
                } else {
                    lul = format!("{}{}", 0, lul);
                    carry = 1;
                }
            },
            _ => panic!()
        }
    }
    if carry == 1 {
        lul = format!("{}{}", 1, lul)
    }
    lul
}