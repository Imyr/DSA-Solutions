pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut lvs = String::new();
    let mut start = None;
    let mut big_stack = vec![];
    let mut small_stack = vec![];
    for (idx, c) in s.chars().enumerate() {
        match c {
            '(' => {
                if start.is_none() {
                    start = Some(idx);
                }
                small_stack.push('(');
                big_stack.push('(');
            },
            ')' => {
                if small_stack.last() == Some(&'(') {
                    big_stack.push(')');
                    small_stack.pop();
                } else {
                    big_stack.clear();
                    small_stack.clear();
                }
            },
            _ => unreachable!(),
        }
    }
    todo!()
}