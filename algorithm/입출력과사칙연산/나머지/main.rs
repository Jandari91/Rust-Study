/*
https://www.acmicpc.net/problem/10430
*/

use std::io;

fn main(){
    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let numbers:Vec<&str> = arr.split_whitespace().collect();
    let a = numbers[0].parse::<i32>().unwrap();
    let b = numbers[1].parse::<i32>().unwrap();
    let c = numbers[2].parse::<i32>().unwrap();

    println!("{}", (a+b)%c);
    println!("{}", ((a%c) + (b%c))%c);
    println!("{}", (a*b)%c);
    println!("{}", ((a%c) * (b%c))%c);
}