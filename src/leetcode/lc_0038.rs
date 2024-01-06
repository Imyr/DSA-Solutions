fn say(input: String) -> String {
    let input = input.chars().collect::<Vec<char>>();
    let mut said = String::new();
    let mut temp = input[0];
    let mut count = 1;
    for i in 1..input.len() {
        if input[i] == temp {
            count += 1;
        } else {
            said = format!("{}{}{}", said, count, temp);
            temp = input[i];
            count = 1;
        }
    }
    said = format!("{}{}{}", said, count, temp);
    said
}   

pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        String::from("1")
    } else {
        say(count_and_say(n-1))
    }
}