#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]

mod codeforces;
mod leetcode;

fn main() {
    let lmao = vec![10,9,2,5,3,7,101,18];
    let lul = leetcode::lc_1235::Solution::job_scheduling(vec![1,2,3,3], vec![3,4,5,6], vec![50,10,40,70]);
    println!("\n-----ANSWER-----\n{:?}\n----------------", lul);
}