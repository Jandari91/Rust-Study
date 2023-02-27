/*
https://www.acmicpc.net/problem/2525
 */

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let numbers:Vec<&str> = str.split_whitespace().collect();
    let mut a = numbers[0].parse::<i32>().unwrap();
    let mut b = numbers[1].parse::<i32>().unwrap();

    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let d = duration.trim().parse::<i32>().unwrap();

    b = b + d;

    let x = b / 60;
    b = b % 60;
    a = (a + x)%24;

    println!("{} {}", a, b);
}