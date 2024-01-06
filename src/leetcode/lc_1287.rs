pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let mut temp = arr[0];
    let mut count = 1;
    for i in 1..arr.len() {
        if count > arr.len()/4 {
            return temp
        }
        if arr[i] != temp {
            temp = arr[i];
            count = 1;
        } else {
            count += 1;
        }
    }   
    temp 
}