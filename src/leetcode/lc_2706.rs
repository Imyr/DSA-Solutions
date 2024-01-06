pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let (mut a1, mut a2) = (101, 101);       
    for e in prices {
        if e <= a1 {
            a2 = a1;
            a1 = e;
        } else if e < a2 {
            a2 = e;
        }
    }    
    if money-(a1+a2) < 0 {money-(a1+a2)} else {money}
}