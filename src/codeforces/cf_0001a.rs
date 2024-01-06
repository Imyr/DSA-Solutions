use std::io;

fn main() {
    let mut iterator = io::stdin().lines();
    let input = iterator.next().unwrap().unwrap();

    let line = input.split(' ').map(|c| c.parse().unwrap()).collect::<Vec<f64>>();
    println!("{}",  (line[0]/line[2]).ceil()*(line[1]/line[2]).ceil());
}