pub fn roman_to_int(mut s: String) -> i32 {
    let one = vec!["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let ten = vec!["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let hun = vec!["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let tho = vec!["M", "MM", "MMM"];

    let mut num = vec![0, 0, 0, 0];

    let mut ones = 0;
    for i in vec![9, 8, 7, 6, 4, 5, 3, 2, 1] {
        if s.find(one[i-1]).is_some() {
            s = s.replace(one[i-1], "");
            ones = i;
            break;
        }
    }
    num[3] = ones;

    let mut tens = 0;
    for i in vec![9, 8, 7, 6, 4, 5, 3, 2, 1] {
        if s.find(ten[i-1]).is_some() {
            s = s.replace(ten[i-1], "");
            tens = i;
            break;
        }
    }
    num[2] = tens;

    let mut huns = 0;
    for i in vec![9, 8, 7, 6, 4, 5, 3, 2, 1] {
        if s.find(hun[i-1]).is_some() {
            s = s.replace(hun[i-1], "");
            huns = i;
            break;
        }   
    }
    num[1] = huns;

    let mut thou = 0;
    for i in vec![3, 2, 1] {
        if s.find(tho[i-1]).is_some() {
            thou = i;
            break;
        }
    }
    num[0] = thou;
    let mut number = 0;
    for i in num {
        number = number*10 + i as i32;
    }
    number
}