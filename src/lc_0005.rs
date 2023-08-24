pub fn longest_palindrome(s: String) -> String {
    let mut palindrome = String::new();
    let mut kek = vec![];
    for i in s.chars() {
        kek.push(i);
    }
    for i in 0..kek.len() {
        let mut pstr1 = format!("{}", kek[i]);
        let mut pstr2 = format!("{}", kek[i]);
        if i+1 < kek.len() {
           if kek[i] == kek[i+1] {
                pstr1 = format!("{}{}", kek[i], kek[i+1]);
                let mut j = 1;
                while (i as i32 - j as i32 >= 0) && (i+1+j < kek.len()) {
                    if kek[i-j] == kek[i+1+j] {
                        pstr1 = format!("{}{}{}", kek[i-j], pstr1, kek[i+1+j]);
                    } else {
                        break;
                    } 
                    j += 1;
                } 
            }
        }
        if i+2 < kek.len() {
            if kek[i] == kek[i+2] {
                pstr2 = format!("{}", kek[i+1]);
                let mut j = 0;
                while (i as i32 - j as i32 >= 0) && (i+2+j < kek.len()) {
                    if kek[i-j] == kek[i+2+j] {
                        pstr2 = format!("{}{}{}", kek[i-j], pstr2, kek[i+2+j]);
                    } else {
                        break;
                    } 
                    j += 1;
                } 
            }
        }
        if (palindrome.len() < pstr1.len()) || (palindrome.len() < pstr2.len()) {
            if pstr1.len() > pstr2.len() {
                palindrome = pstr1;
            } else {
                palindrome = pstr2;
            }
        }
    }
    return palindrome;
}