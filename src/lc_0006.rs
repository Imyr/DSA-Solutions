pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
            return s;
        }
    let mut lmao = String::new();
    let num_rows = num_rows as usize;
    let mut kek = vec![];
    for hmm in s.chars() {
        kek.push(hmm);
    };
    let i = num_rows*2 - 2;
    let mut j1 = 0;
    while j1 < kek.len() {
        lmao.push(kek[j1]);
        j1 += i;
    };
    let mut k = 1;
    while (k <= num_rows - 2) && (k < kek.len()) {
        lmao.push(kek[k]);
        let mut j2 = 1;
        while i*j2-k < kek.len() {
            lmao.push(kek[i*j2-k]);
            if i*j2+k < kek.len(){ 
                lmao.push(kek[i*j2+k]);
            }
            j2 += 1;
        }
        k += 1;
    };
    let mut j3 = num_rows-1;
    while j3 < kek.len() {
        lmao.push(kek[j3]);
        j3 += i;
    };
    lmao
}