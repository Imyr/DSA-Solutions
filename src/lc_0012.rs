fn q_r(dividend: usize, divisor: usize) -> (usize, usize) {
    (dividend/divisor, dividend%divisor) 
}

fn recurse(dividend: usize, mut divisors: Vec<usize>) -> Vec<usize> {
    match divisors.is_empty() {
        true => vec![],
        false => {
            let (value, new_dividend) = q_r(dividend, divisors.pop().expect("kya bsdk"));
            let mut remainders= vec![value];
            remainders.extend(recurse(new_dividend, divisors));  
            remainders
        }
    }
}

fn romanize(weights: (usize, usize), letters: (String, String, String)) -> String {
    let (a, b, c) = letters;
    match weights {
        (0, d) => {
            if d==4 {
                format!("{}{}", a, b)
            } else {
                format!("{}", a.repeat(d))
            }
        },
        (1, d) => {
            if d==4 {
                format!("{}{}", a, c)
            } else {
                format!("{}{}", b, a.repeat(d))
            }
        },
        _ => String::new(), 
    }
}

pub fn int_to_roman(num: i32) -> String {
    let weights = recurse(num as usize, vec![1, 5, 10, 50, 100, 500, 1000]);

    "M".repeat(weights[0])
    + &romanize((weights[1], weights[2]), ("C".to_string().to_string(), "D".to_string().to_string(), "M".to_string().to_string()))
    + &romanize((weights[3], weights[4]), ("X".to_string().to_string(), "L".to_string().to_string(), "C".to_string().to_string()))
    + &romanize((weights[5], weights[6]), ("I".to_string().to_string(), "V".to_string().to_string(), "X".to_string().to_string()))
}