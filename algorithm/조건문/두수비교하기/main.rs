/*
https://www.acmicpc.net/problem/1330
 */

use std::io;
fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let numbers:Vec<&str> = str.split_whitespace().collect();
    let a = numbers[0].parse::<i32>().unwrap();
    let b = numbers[1].parse::<i32>().unwrap();

    if a > b {
        println!(">");
    }else if a < b {
        println!("<");
    }else{
        println!("==")
    }

}