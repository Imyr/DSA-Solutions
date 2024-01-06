pub fn largest_good_integer(num: String) -> String {
    let mut max = -1;
    let num = num.chars().collect::<Vec<char>>();
    for idx in 1..=num.len()-2 {
        if num[idx]==num[idx-1] && num[idx]==num[idx+1] {
            let number = (0..3).map(|_| num[idx]).collect::<String>().parse::<i32>().unwrap();
            if number > max {
                max = number;
            }
        }
    }

    if max == -1 {
        return String::new()
    } else if max == 0 {
        return String::from("000")
    } else {
        format!("{}", max)
    }
}