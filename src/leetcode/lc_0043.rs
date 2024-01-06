pub fn add(mut one: Vec<u32>, mut two: Vec<u32>) -> Vec<u32> {
    let mut sum = vec![];
    let mut carry = 0;
    while !(one.is_empty() || two.is_empty()) {
        let s = one.pop().unwrap() + two.pop().unwrap() + carry;
        sum.push(s%10);
        carry = s/10;
    }
    while !one.is_empty() {
        let s = one.pop().unwrap() + carry;
        sum.push(s%10);
        carry = s/10;
    }
    while !two.is_empty() {
        let s = two.pop().unwrap() + carry;
        sum.push(s%10);
        carry = s/10;
    }
    if carry != 0 {
        sum.push(carry);
    }
    sum.reverse();
    sum
}

pub fn mul(mut one: Vec<u32>, two: u32) -> Vec<u32> {
    let mut product = vec![];
    let mut carry = 0;
    while !one.is_empty() {
        let p = one.pop().unwrap() * two + carry;
        product.push(p%10);
        carry = p/10;
    }
    if carry != 0 {
        product.push(carry);
    }
    product.reverse();
    product
}

pub fn multiply(num1: String, num2: String) -> String {
    if num1==String::from("0") || num2==String::from("0") {
        return "0".to_owned();
    } 
    let num1 = num1.chars().map(|ascii | ascii.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let mut num2 = num2.chars().map(|ascii | ascii.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut final_product = vec![0];
    let mut zeroes = 0;
    while !num2.is_empty() {
        let multiplier = num2.pop().unwrap();
        let mut product = mul(num1.clone(), multiplier);
        for _ in 0..zeroes {
            product.push(0);
        }
        final_product = add(final_product, product);
        zeroes += 1;
    }

    final_product.iter().map(|d| d.to_string()).collect::<String>()
}