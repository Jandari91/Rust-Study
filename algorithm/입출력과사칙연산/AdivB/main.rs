/*
https://www.acmicpc.net/problem/1008
*/

use std::io;
fn main(){
    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let numbers:Vec<&str> = arr.split_whitespace().collect();
    let a = numbers[0].parse::<f64>().unwrap();
    let b = numbers[1].parse::<f64>().unwrap();

    println!("{}", a/b);
}