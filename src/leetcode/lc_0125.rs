pub fn is_palindrome(s: String) -> bool {
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_lowercase().to_string()).collect();
    s.chars().eq(s.chars().rev())
}   