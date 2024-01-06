pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();
    let mut i = 0;
    let mut j = 0;
    let mut content = 0;
    while i <= g.len() && j <= s.len() {
        if g[i] <= s[j] {
            content += 1;
            i += 1;
            j += 1;       
        } else {
            j += 1;
        }
    }
    content
}