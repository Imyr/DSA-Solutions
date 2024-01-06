fn romanize(weights: (usize, usize), letters: (&str, &str, &str)) -> String {
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

pub fn int_to_roman(mut num: i32) -> String {
    let mut weights: Vec<usize> = vec![];
    for divisor in vec![1000, 500, 100, 50, 10, 5, 1] {
        weights.push((num/divisor) as usize);
        num%=divisor;
    }
    "M".repeat(weights[0])
    + &romanize((weights[1], weights[2]), ("C", "D", "M"))
    + &romanize((weights[3], weights[4]), ("X", "L", "C"))
    + &romanize((weights[5], weights[6]), ("I", "V", "X"))
}