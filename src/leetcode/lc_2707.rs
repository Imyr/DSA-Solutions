pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut indices= vec![0; s.len()]; 
    for word in dictionary {
        let word: Vec<char> = word.chars().collect();
        if word.len() > s.len() {
            break;
        }
        for i in 0..s.len() {
            if word[0] == s[i] {
                let mut yes = true;
                for j in 1..word.len() {
                    if s.len()<=i+j {
                        break;
                    }
                    if word[j]!=s[i+j] {
                        yes = false;
                        break;
                    }
                }
                if yes {
                    for j in i..i+word.len() {
                        indices[j] = 1;
                    }
                    println!("{:?}", indices);
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..indices.len() {
        if indices[i] == 0 {
            count+=1;
        }
    }
    count
}