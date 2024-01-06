pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut operand_stack: Vec<i32> = vec![]; 
    for c in tokens {
        match c.as_str() {
            "+" => {
                let a = operand_stack.pop().unwrap();
                let b = operand_stack.pop().unwrap();
                operand_stack.push(b + a);
            },
            "-" => {
                let a = operand_stack.pop().unwrap();
                let b = operand_stack.pop().unwrap();
                operand_stack.push(b - a);
            },
            "*" => {
                let a = operand_stack.pop().unwrap();
                let b = operand_stack.pop().unwrap();
                operand_stack.push(b * a);
            },
            "/" => {
                let a = operand_stack.pop().unwrap();
                let b = operand_stack.pop().unwrap();
                operand_stack.push(b / a);
            },
            _ => operand_stack.push(c.parse().unwrap()),
        }
    }
    operand_stack.pop().unwrap()
}