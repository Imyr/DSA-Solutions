use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("()"));
    for _ in 1..n {
        let mut lmao: HashSet<String> = HashSet::new();
        for s in set {
            for i in 0..s.len(){
                let mut hmm = s.clone();
                hmm.insert_str(i, "()");
                lmao.insert(hmm);
            }
        }
        set = lmao;
    }
    set.into_iter().collect()
}