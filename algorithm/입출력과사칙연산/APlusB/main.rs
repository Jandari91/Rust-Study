/*
https://www.acmicpc.net/problem/1000
*/

use std::io;
fn main() {
    let mut arr = String::new();
    
    io::stdin().read_line(&mut arr).unwrap();
    
    let numbers: Vec<&str> = arr.split_whitespace().collect();
    
    let first_num = numbers[0].parse::<i32>().unwrap();
    let second_num = numbers[1].parse::<i32>().unwrap();

    println!("{}", first_num + second_num);
}