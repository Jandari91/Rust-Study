/*
https://www.acmicpc.net/problem/2884
 */

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let numbers : Vec<&str> = str.split_whitespace().collect();
    let mut h = numbers[0].parse::<i32>().unwrap();
    let mut m = numbers[1].parse::<i32>().unwrap();

    m = m - 45;
    if m < 0 {
        m = 60 + m;
        h = h - 1;
    }

    if h < 0 {
        h = 24 + h;
    }

    println!("{} {}", h, m);

}