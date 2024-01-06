pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();
    if haystack.len() < needle.len() {
        return -1
    }
    for i in 0..haystack.len() {
        if haystack[i] == needle[0] {
            let mut yes = true;
            for j in 1..needle.len() {
                if i+j>=haystack.len() {
                    return -1
                }
                if haystack[i+j]!=needle[j] {
                    yes = false;
                    break;
                }
            }
            if yes {
                return i as i32
            }
        }
    }
    -1
}